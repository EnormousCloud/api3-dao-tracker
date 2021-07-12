pub mod args;
pub mod dumper;
pub mod endpoints;
pub mod inject;
pub mod pool;
pub mod reader;

use args::DumpMode;
use client::events;
use client::state::{AppState, OnChainEvent};
use futures::{FutureExt, StreamExt};
use std::collections::HashMap;
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
    pub fn new(subscribers: Subscribers) -> Self {
        Self {
            subscribers,
            verbose: false,
            app: AppState::new(),
        }
    }
}

impl reader::EventHandler for State {
    fn on(&mut self, entry: events::Api3, log: web3::types::Log) -> () {
        if self.verbose {
            // it becomes verbose in watching mode
            tracing::info!("{:?}", entry);
        }
        log.block_number.map(|block_number| {
            self.app.last_block = block_number.as_u64();
        });
        entry.get_wallets().iter().for_each(|wallet| {
            if !self.app.wallets.contains_key(&wallet) {
                // tracing::info!("{:?} {:?}", wallet, entry);
                self.app.wallets.insert(wallet.clone(), vec![]);
            }
            self.app
                .wallets
                .get_mut(&wallet)
                .unwrap()
                .push(OnChainEvent {
                    entry: entry.clone(),
                    log: log.clone(),
                });
        });
        entry.get_voting().map(|id| {
            if !self.app.votings.contains_key(&id) {
                self.app.votings.insert(id, vec![]);
            }
            self.app.votings.get_mut(&id).unwrap().push(OnChainEvent {
                entry: entry.clone(),
                log: log.clone(),
            });
        });
        if self.verbose {
            futures::executor::block_on(async {
                let list = self.subscribers.read().await;
                // tracing::info!("sending to {:?} subscribers", list.len());
                // broadcasting event to all subscribers
                for (&subscriber_id, tx) in list.iter() {
                    tracing::debug!("<sent to #{}> {:?}", subscriber_id, entry);
                    if let Err(err) = tx.send(Ok(Message::text(format!("{:?}", entry)))) {
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

    let scanner = reader::Scanner::new(
        vec![addr_voting1, addr_agent1],
        vec![addr_voting2, addr_agent2],
        vec![addr_pool, addr_convenience],
        args.genesis_block,
        args.rpc_batch_size,
    );

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
    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let subscribers = Subscribers::default();
    let state = Arc::new(Mutex::new(State::new(subscribers.clone())));

    // Turn our "state" into a new Filter...
    let subscribers = warp::any().map(move || subscribers.clone());
    let last_block = {
        let rc = state.clone();
        let last_block = scanner.scan(&web3, &mut *rc.lock().unwrap()).await?;
        let s = rc.lock().unwrap();
        tracing::info!(
            "found: {} wallets, {} votings",
            s.app.wallets.len(),
            s.app.votings.len()
        );
        last_block
    };

    let socket_addr: std::net::SocketAddr = args.listen.parse().expect("invalid bind to listen");
    if args.watch {
        // This is unstable so far
        let rc = state.clone();
        rc.lock().unwrap().verbose = true;
        let rc = state.clone();
        tokio::spawn(async move {
            scanner.watch(&web3, last_block, rc).await.unwrap();
        });
        let chat = warp::path("ws").and(warp::ws()).and(subscribers).map(
            |ws: warp::ws::Ws, subscribers| {
                ws.on_upgrade(move |socket| ws_connected(socket, subscribers))
            },
        );
        let routes = endpoints::routes(args.static_dir.clone(), state).or(chat);
        warp::serve(routes.with(warp::trace::request())).run(socket_addr).await;
    } else {
        let routes = endpoints::routes(args.static_dir.clone(), state);
        warp::serve(routes.with(warp::trace::request())).run(socket_addr).await;
    }
    Ok(())
}
