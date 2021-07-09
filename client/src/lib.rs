pub mod events;
pub mod logreader;
pub mod nice;
pub mod state;

use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    // pub chains: Option<Vec<NetworkInfo>>,
}
impl App {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>"API3 DAO Tracker"</h1>
            </main>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}

#[wasm_bindgen]
pub fn main(_serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let app = App::new();
    // let mut app = App::new();
    // if serialized_state.len() > 4 {
    //     match serde_json::from_str::<App>(&serialized_state) {
    //         Ok(state) => {
    //             // app.chains = state.chains;
    //             // info!("parsing ok {:?}", app.chains);
    //         }
    //         Err(e) => {
    //             info!("parsing error {}", e);
    //         }
    //     };
    // }
    match web_sys::window() {
        Some(window) => {
            let document = window.document().expect("should have a document on window");
            Program::new_replace_mount(
                app,
                &document.query_selector_all("main").unwrap().get(0).unwrap(),
            );
        }
        None => {
            trace!("window not found");
            Program::mount_to_body(app);
        }
    }
}
