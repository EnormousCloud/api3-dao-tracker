pub mod components;
pub mod events;
pub mod logreader;
pub mod nice;
pub mod routes;
pub mod screens;
pub mod state;

use routes::Switch;
use sauron::prelude::*;
use state::AppState;

#[macro_use]
extern crate log;

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let mut routes = Switch::new();
    if serialized_state.len() > 4 {
        match serde_json::from_str::<AppState>(&serialized_state) {
            Ok(state) => {
                info!("parsing ok {:?}", state);
                routes = Switch::from_state(state);
            }
            Err(e) => {
                info!("parsing error {}", e);
            }
        };
    }
    let document = sauron::dom::document();
    Program::new_replace_mount(
        routes,
        &document.query_selector_all("main").unwrap().get(0).unwrap(),
    );
}
