pub mod components;
pub mod events;
pub mod eventsnode;
pub mod logreader;
pub mod nice;
pub mod router;
pub mod screens;
pub mod state;

use sauron::prelude::*;
use state::AppState;
use std::str::FromStr;
use web3::types::H160;

#[macro_use]
extern crate log;

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let mut appstate = AppState::new(1);
    if serialized_state.len() > 4 {
        match serde_json::from_str::<AppState>(&serialized_state) {
            Ok(state) => {
                info!("parsing state ok");
                appstate = state;
            }
            Err(e) => {
                info!("parsing error {}", e);
            }
        };
    }
    let document = sauron::dom::document();
    let root = document.query_selector_all("main").unwrap().get(0).unwrap();
    let pathname = sauron::dom::window()
        .location()
        .pathname()
        .expect("cannot get window.location");
    match pathname.as_str() {
        "/votings" => {
            Program::new_replace_mount(screens::votings::Screen::new(appstate), &root);
        }
        "/rewards" => {
            Program::new_replace_mount(screens::rewards::Screen::new(appstate), &root);
        }
        "/wallets" => {
            Program::new_replace_mount(screens::wallets::Screen::new(appstate), &root);
        }
        _ => {
            if pathname.starts_with("/votings/") {
                let offs = "/votings/".len();
                let vote_str: String = pathname.chars().skip(offs).collect();
                let vote_ref = vote_str.parse::<u64>().unwrap();
                Program::new_replace_mount(screens::voting::Screen::new(appstate, vote_ref), &root);
            } else if pathname.starts_with("/wallets/") {
                let offs = "/wallets/".len();
                let wallet_str: String = pathname.chars().skip(offs).collect();
                let addr = H160::from_str(&wallet_str).unwrap();
                Program::new_replace_mount(screens::wallet::Screen::new(appstate, &addr), &root);
            } else {
                Program::new_replace_mount(screens::home::Screen::new(appstate), &root);
            }
        }
    };
}
