// use crate::components::themeswitcher;
use crate::screens::home;
use crate::state::AppState;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
// use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Switch {
    /// server side state
    state: AppState,
    screen_home: Option<home::Screen>,
}

impl Switch {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            screen_home: Some(home::Screen::new(AppState::new())),
        }
    }
    pub fn from_state(app: AppState) -> Self {
        Self {
            state: app.clone(),
            screen_home: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    OnMounted,
}

impl Component<Msg> for Switch {
    fn view(&self) -> Node<Msg> {
        // let _ = themeswitcher::init();
        if let Some(s) = &self.screen_home {
            return s.view().map_msg(move |_| Msg::OnMounted);
        }
        return div(vec![], vec![]);
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("routes: UPDATE: {:?}", msg);
        Cmd::none()
    }
}
