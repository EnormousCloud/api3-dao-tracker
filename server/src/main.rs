pub mod args;
pub mod contracts;
pub mod dumper;
pub mod endpoints;
pub mod ens;
pub mod inject;
pub mod reader;
pub mod treasury;

use args::DumpMode;
use client::state::{AppState, OnChainEvent};
use futures::{FutureExt, StreamExt};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use web3::types::H160;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
type Subscribers = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[derive(Debug, Clone)]
pub struct State {
    /// whether to log incoming messages
    pub verbose: bool,
    /// subscribers of the chat
    pub subscribers: Subscribers,
    /// client application state
    pub app: AppState,
}

impl State {
    pub fn new(subscribers: Subscribers, chain_id: u64) -> Self {
        Self {
            subscribers,
            verbose: false,
            app: AppState::new(chain_id),
        }
    }
}

impl reader::EventHandler for State {
    fn on(&mut self, e: OnChainEvent, log: web3::types::Log) -> () {
        if self.verbose {
            // it becomes verbose in watching mode
            tracing::info!("{}", serde_json::to_string(&e).unwrap());
        }
        self.app.update(e.clone(), log);
        if self.verbose {
            futures::executor::block_on(async {
                let list = self.subscribers.read().await;
                // tracing::info!("sending to {:?} subscribers", list.len());
                // broadcasting event to all subscribers
                for (&subscriber_id, tx) in list.iter() {
                    let json_msg = serde_json::to_string(&e).unwrap();
                    tracing::debug!("<sent to #{}> {}", subscriber_id, json_msg);
                    if let Err(err) = tx.send(Ok(Message::text(json_msg))) {
                        tracing::warn!("<disconnected #{}> {}", subscriber_id, err);
                    }
                }
            });
        }
    }
}

async fn ws_connected(ws: WebSocket, subscribers: Subscribers) {
    // Use a counter to assign a new unique ID for this user.
    let subscriber_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    tracing::info!("connecting {}", subscriber_id);

    // Split the socket into a sender and receive of messages.
    let (ws_tx, mut ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            tracing::warn!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected users.
    subscribers.write().await.insert(subscriber_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the subscriber sends a message, broadcast it to
    // all other users...
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                tracing::warn!("websocket error(uid={}): {}", subscriber_id, e);
                break;
            }
        };
        tracing::debug!("message from user {:?}", msg);
    }

    // ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    ws_disconnected(subscriber_id, &subscribers).await;
}

async fn ws_disconnected(subscriber_id: usize, subscribers: &Subscribers) {
    // Stream closed up, so remove from the user list
    subscribers.write().await.remove(&subscriber_id);

    let s = subscribers.read().await;
    tracing::info!("disconnected {}, {} online", subscriber_id, s.len());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => return Err(anyhow::Error::msg(format!("Args parsing error {}", e))),
    };
    let addr_pool = H160::from_str(args.address_api3_pool.as_str()).expect("ADDR_API3_POOL");
    let addr_token = H160::from_str(args.address_api3_token.as_str()).expect("ADDR_API3_TOKEN");
    let addr_usdc_token =
        H160::from_str(args.address_usdc_token.as_str()).expect("ADDR_USDC_TOKEN");
    let addr_convenience =
        H160::from_str(args.address_convenience.as_str()).expect("ADDR_API3_CONVENIENCE");
    let addr_voting1 =
        H160::from_str(args.address_voting1.as_str()).expect("ADDR_API3_VOTING_PRIMARY");
    let addr_agent1 =
        H160::from_str(args.address_agent1.as_str()).expect("ADDR_API3_AGENT_PRIMARY");
    let addr_voting2 =
        H160::from_str(args.address_voting2.as_str()).expect("ADDR_API3_VOTING_SECONDARY");
    let addr_agent2 =
        H160::from_str(args.address_agent2.as_str()).expect("ADDR_API3_AGENT_SECONDARY");

    let mut treasury_tokens: BTreeMap<String, H160> = BTreeMap::new();
    treasury_tokens.insert("USDC".into(), addr_usdc_token);
    treasury_tokens.insert("API3".into(), addr_token);

    if let Some(_) = args.rpc_endpoint.find("http://") {
        return Err(anyhow::Error::msg(
            "only IPC endpoint is allowed. No real time events tracking with HTTP",
        ));
    } else if let Some(_) = args.rpc_endpoint.find("https://") {
        return Err(anyhow::Error::msg(
            "only IPC endpoint is allowed. No real time events tracking with HTTPS",
        ));
    }
    if !Path::new(args.rpc_endpoint.as_str()).exists() {
        return Err(anyhow::Error::msg("IPC file doesn't exists"));
    }
    let transport = web3::transports::Ipc::new(args.rpc_endpoint.as_str())
        .await
        .expect("Failed to connect to IPC");
    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?.as_u64();

    let mut addresses = vec![addr_pool, addr_convenience];
    if let Some(address_api3_circulation) = args
        .address_api3_circulation
        .map(|x| H160::from_str(&x).expect("ADDR_API3_CIRCULATION"))
    {
        addresses.push(address_api3_circulation);
    }

    let mut scanner = reader::Scanner::new(
        chain_id,
        args.cache_dir.as_str(),
        vec![addr_voting1, addr_agent1],
        vec![addr_voting2, addr_agent2],
        addresses,
        args.genesis_block,
        args.max_block,
        args.rpc_batch_size,
    );

    let socket_addr: std::net::SocketAddr = args.listen.parse().expect("invalid bind to listen");
    // starting a "loading" only server
    // and do not start if we are in dump-mode
    let loading_server = match args.dump {
        None => Some(tokio::spawn(async move {
            let routes = endpoints::routes_loading();
            warp::serve(routes.with(warp::trace::request()))
                .run(socket_addr)
                .await;
        })),
        _ => None,
    };

    if let Some(mode) = &args.dump {
        match mode {
            DumpMode::Unknown => {
                let mut dumper = dumper::Unknown::new();
                scanner.scan(&web3, &mut dumper).await?;
                dumper.done();
            }
            DumpMode::Events => {
                let mut dumper = dumper::Events::new();
                scanner.scan(&web3, &mut dumper).await?;
            }
        };
        std::process::exit(0);
    }

    let addr_circulation: Option<H160> = args
        .address_circulation
        .map(|x| H160::from_str(x.as_str()).expect("ADDR_API3_CIRCULATION"));

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let subscribers = Subscribers::default();
    let server_state = State::new(subscribers.clone(), chain_id);
    let state = Arc::new(Mutex::new(server_state));

    let mut treasury_wallets: BTreeMap<String, H160> = BTreeMap::new();
    treasury_wallets.insert("Primary".into(), addr_agent1);
    treasury_wallets.insert("Secondary".into(), addr_agent2);

    // Turn our "state" into a new Filter...
    let subscribers = warp::any().map(move || subscribers.clone());
    let last_block = {
        let rc = state.clone();
        let last_block = scanner.scan(&web3, &mut *rc.lock().unwrap()).await?;
        let mut s = rc.lock().unwrap();
        tracing::info!(
            "found: {} wallets, {} votings",
            s.app.wallets.len(),
            s.app.votings.len()
        );
        s.app.pool_info = crate::contracts::Pool::new(&web3, addr_pool).read().await;
        tracing::info!("pool info {:?}", s.app.pool_info);
        if let Some(addr_supply) = addr_circulation {
            s.app.circulation = crate::contracts::Supply::new(
                &web3,
                addr_supply,
                addr_token,
                addr_convenience,
                addr_voting1,
                addr_voting2,
            )
            .read()
            .await;
            tracing::info!("circulation info {:?}", s.app.circulation);
            if let Some(ci) = &s.app.circulation {
                treasury_wallets.insert("V1 Treasury".into(), ci.addr_v1_treasury);
            }
        }

        s.app.treasuries =
            crate::treasury::read_treasuries(&web3, &treasury_tokens, &treasury_wallets).await;
        tracing::info!("treasuries {:?}", s.app.treasuries);

        // re-read votings and extract static data for votes
        let conv = crate::contracts::Convenience::new(&web3, addr_convenience);
        for (_, v) in &mut s.app.votings {
            let static_data = conv
                .get_voting_static_data(v.primary, v.creator, v.vote_id)
                .await;
            println!("voting_static_data = {:?}", static_data);
            if let Some(data) = static_data {
                v.votes_total = data.voting_power; // adjust with precise #
                v.static_data = Some(data.clone());
            }
        }
        last_block
    };
    if !args.no_ens {
        let ens = crate::ens::ENS::new(web3.clone(), args.cache_dir.as_str());
        let rc = state.clone();
        let mut s = rc.lock().unwrap();
        for (addr, w) in &mut s.app.wallets {
            if let Some(name) = ens.name(addr.clone()).await {
                tracing::info!("ENS for {:?} is {:?}", addr, name);
                w.ens = Some(name);
            };
        }
        tracing::info!("done with ENS");
    }

    loading_server.map(|server| {
        server.abort();
        std::thread::sleep(std::time::Duration::from_secs(1)); // wait for server to shutdown
    });

    if args.watch {
        let w3 = web3.clone();
        let rc = state.clone();
        rc.lock().unwrap().verbose = true;
        let rc = state.clone();
        tokio::spawn(async move {
            scanner.watch_ipc(&web3, last_block, rc).await.unwrap();
        });

        // one more thread fto update ppol and circulation hourly
        if let Some(addr_supply) = addr_circulation {
            let rc = state.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));
                let contract_pool = crate::contracts::Pool::new(&w3, addr_pool.clone());
                let contract_circulation = crate::contracts::Supply::new(
                    &w3,
                    addr_supply,
                    addr_token,
                    addr_convenience,
                    addr_voting1,
                    addr_voting2,
                );
                loop {
                    interval.tick().await; // wait an hour
                    if let Some(pool) = contract_pool.read().await {
                        tracing::info!("pool info {:?}", pool);
                        let mut s = rc.lock().unwrap();
                        s.app.pool_info = Some(pool);
                    } else {
                        tracing::info!("pool info - failed to update");
                    }
                    if let Some(circulation) = contract_circulation.read().await {
                        tracing::info!("circulation info {:?}", circulation);
                        let mut s = rc.lock().unwrap();
                        s.app.circulation = Some(circulation);
                    } else {
                        tracing::info!("circulation info - failed to update");
                    }
                }
            });
        }

        let chat = warp::path("ws").and(warp::ws()).and(subscribers).map(
            |ws: warp::ws::Ws, subscribers| {
                ws.on_upgrade(move |socket| ws_connected(socket, subscribers))
            },
        );
        let routes = endpoints::routes(args.static_dir.clone(), state).or(chat);
        warp::serve(routes.with(warp::trace::request()))
            .run(socket_addr)
            .await;
    } else {
        let routes = endpoints::routes(args.static_dir.clone(), state);
        warp::serve(routes.with(warp::trace::request()))
            .run(socket_addr)
            .await;
    }
    Ok(())
}
