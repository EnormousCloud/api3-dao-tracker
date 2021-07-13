use crate::inject;
use client::screens;
use client::state::AppState;
use sauron::prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::info;
use warp::Filter;
use warp::Reply;
use web3::types::H160;

pub fn render_html(
    static_dir: &str,
    app: &AppState,
    component: Box<dyn Render>,
) -> impl warp::Reply {
    let file = format!("{}/index.html", static_dir);
    info!("content file {:?}", file);
    let content = std::fs::read_to_string(file.as_str()).expect("index.html not found");
    info!("content size {:?}", content.len());
    let state_json = serde_json::to_string(app).expect("state could not be converted to JSON");
    let mut state_html = String::new();
    let rendered: String = match component.render(&mut state_html) {
        Ok(_) => {
            let c1 = inject::it(content.as_str(), "<main>", "</main>", &state_html);
            inject::replace(c1.as_str(), "main(`", "`)", "") // call to main function is removed
        }
        Err(_) => inject::it(content.as_str(), "main(`", "`)", &state_json),
    };
    info!("rendered {:?}", rendered.len());
    warp::reply::html(rendered)
}

pub fn render_err(static_dir: &str, app: &AppState, msg: &'static str) -> warp::reply::Response {
    let screen = screens::failure::Screen {
        msg: msg.to_owned(),
        state: app.clone(),
    };
    warp::reply::with_status(
        render_html(static_dir.to_owned().as_str(), app, Box::new(screen.view())),
        warp::http::StatusCode::BAD_REQUEST,
    )
    .into_response()
}

pub fn json_error(msg: &str) -> warp::reply::Response {
    let mut res: BTreeMap<String, String> = BTreeMap::new();
    res.insert("error".to_owned(), msg.to_string());
    let body = warp::reply::json(&res);
    warp::reply::with_status(body, warp::http::StatusCode::BAD_REQUEST).into_response()
}

pub fn wrap_result<T>(result: &T) -> BTreeMap<String, T>
where
    T: Clone,
{
    let mut res = BTreeMap::new();
    res.insert("result".to_owned(), result.clone());
    res
}

pub fn routes(
    static_dir: String,
    state: Arc<Mutex<crate::State>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let dir = static_dir.clone();

    let api_state = warp::path!("api" / "state").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            warp::reply::json(&state.app)
        }
    });
    let api_wallets = warp::path!("api" / "wallets").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            warp::reply::json(&wrap_result(&state.app.wallets))
        }
    });
    let api_wallet = warp::path!("api" / "wallets" / String).map({
        let state_rc = state.clone();
        move |id: String| {
            let state = state_rc.lock().unwrap();
            if let Ok(addr) = H160::from_str(id.clone().as_str()) {
                if let Some(w) = state.app.wallets.get(&addr) {
                    warp::reply::json(&wrap_result(&w)).into_response()
                } else {
                    json_error("Not a member of the DAO")
                }
            } else {
                json_error("Invalid Ethereum address")
            }
        }
    });
    let api_votings = warp::path!("api" / "votings").map({
        let state_rc = state.clone();
        move || {
            let state = state_rc.lock().unwrap();
            warp::reply::json(&wrap_result(&state.app.votings))
        }
    });
    let api_voting = warp::path!("votings" / String).map({
        let state_rc = state.clone();
        move |id: String| {
            let (agent, vote_id) = client::events::voting_from_str(&id);
            let vote_ref = client::events::voting_to_u64(&agent, vote_id);
            let state = state_rc.lock().unwrap();
            if let Some(v) = state.app.votings.get(&vote_ref) {
                warp::reply::json(&wrap_result(&v)).into_response()
            } else {
                json_error("Invalid voting ID")
            }
        }
    });
    let api = api_state
        .or(api_wallets)
        .or(api_wallet)
        .or(api_votings)
        .or(api_voting);

    let wallets = warp::path!("wallets").map({
        let state_rc = state.clone();
        let d = dir.clone();
        move || {
            let state = state_rc.lock().unwrap();
            let screen = screens::wallets::Screen {
                state: state.clone().app,
            };
            render_html(&d, &state.app, Box::new(screen.view()))
        }
    });
    let votings = warp::path!("votings").map({
        let state_rc = state.clone();
        let d = dir.clone();
        move || {
            let state = state_rc.lock().unwrap();
            let screen = screens::votings::Screen {
                state: state.clone().app,
            };
            render_html(&d, &state.app, Box::new(screen.view())).into_response()
        }
    });
    let wallet = warp::path!("wallets" / String).map({
        let state_rc = state.clone();
        let d = dir.clone();
        move |id: String| {
            let state = state_rc.lock().unwrap();
            if let Ok(addr) = H160::from_str(id.clone().as_str()) {
                if let Some(_) = state.app.wallets.get(&addr) {
                    let screen = screens::wallet::Screen {
                        addr,
                        state: state.clone().app,
                    };
                    render_html(&d, &state.app, Box::new(screen.view())).into_response()
                } else {
                    render_err(&d, &state.app, "Not a member of the DAO")
                }
            } else {
                render_err(&d, &state.app, "Invalid Ethereum address")
            }
        }
    });
    let voting = warp::path!("votings" / String).map({
        let state_rc = state.clone();
        let d = dir.clone();
        move |id: String| {
            let (agent, vote_id) = client::events::voting_from_str(&id);
            let vote_ref = client::events::voting_to_u64(&agent, vote_id);
            let state = state_rc.lock().unwrap();
            if let Some(_) = state.app.votings.get(&vote_ref) {
                let screen = screens::voting::Screen {
                    vote_ref,
                    vote_id,
                    agent,
                    state: state.clone().app,
                };
                render_html(&d, &state.app, Box::new(screen.view())).into_response()
            } else {
                render_err(&d, &state.app, "Invalid voting ID")
            }
        }
    });
    let home = warp::path::end()
        .map({
            let state_rc = state.clone();
            let d = dir.clone();
            move || {
                let state = state_rc.lock().unwrap();
                let screen = screens::home::Screen {
                    state: state.clone().app,
                };
                render_html(&d, &state.app, Box::new(screen.view())).into_response()
            }
        })
        .or(warp::fs::dir(static_dir.clone()));

    let pages = home.or(wallet).or(wallets).or(voting).or(votings);
    let liveness = warp::path!("_liveness").map(|| format!("# API3 DAO Tracker"));
    liveness.or(api).or(pages)
}
