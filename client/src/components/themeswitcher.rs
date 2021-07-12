use sauron::prelude::*;

const STORAGE_KEY: &'static str = "theme";

const DARK: &'static str = "
:root {
    --color-bk: #333333;
    --color-bk-highlight: #666;
    --color-text: #ddd;
    --color-link: #ffe;
    --color-accent: lightblue;
    --color-body: #000;
    --color-well: #666;
    --color-error: #db3742;
    --color-success: #50b83c;
    --color-success-dark: #329b1e;
    --color-grey: #8c8d9c;
    --color-grey-light: #ced3dc;
}
";

const LIGHT: &'static str = "
:root {
    --color-bk: #fff;
    --color-bk-highlight: #eee;
    --color-text: #000;
    --color-link: blue;
    --color-accent: darkblue;
    --color-body: #fff;
    --color-well: #ccc;
    --color-error: #db3742;
    --color-success: #50b83c;
    --color-success-dark: #329b1e;
    --color-grey: #8c8d9c;
    --color-grey-light: #ced3dc;
}
";

pub struct Switcher {
    pub is_light: bool,
}

impl Switcher {
    pub fn new() -> Self {
        Self {
            is_light: is_light(),
        }
    }
}

pub fn is_light() -> bool {
    let window = match web_sys::window() {
        Some(x) => x,
        None => return false,
    };
    let local_storage = match window.local_storage() {
        Ok(x) => match x {
            Some(x) => x,
            None => return false,
        },
        Err(_) => return false,
    };
    match local_storage.get_item(STORAGE_KEY).unwrap() {
        Some(theme) => theme == "light",
        _ => false,
    }
}

pub fn init() -> () {
    let doc = sauron::dom::document();
    doc.get_element_by_id("color-theme").map(|elem| {
        elem.set_inner_html(if is_light() { LIGHT } else { DARK });
    });
}

pub fn toggle_theme() -> () {
    let window = match web_sys::window() {
        Some(x) => x,
        None => return (),
    };
    let local_storage = match window.local_storage() {
        Ok(x) => match x {
            Some(x) => x,
            None => return (),
        },
        Err(_) => return (),
    };
    let doc = sauron::dom::document();
    doc.get_element_by_id("color-theme").map(|elem| {
        elem.set_inner_html(if elem.inner_html() == LIGHT {
            let _ = local_storage.set_item(STORAGE_KEY, "dark");
            DARK
        } else {
            let _ = local_storage.set_item(STORAGE_KEY, "light");
            LIGHT
        });
    });
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    OnToggle,
}

impl Component<Msg> for Switcher {
    fn view(&self) -> Node<Msg> {
        node! {
            <button
                class="theme-switcher"
                on_click=|_| { toggle_theme(); Msg::OnToggle }
            >
                "SWITCH THEME"
            </button>
        }
    }

    fn init(&self) -> Cmd<Self, Msg> {
        info!("INITIALIZED");
        Cmd::none()
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        info!("UPDATED");
        Cmd::none()
    }
}
