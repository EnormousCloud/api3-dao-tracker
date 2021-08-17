pub mod footer;
pub mod header;
pub mod panel;
pub mod target;

// pub mod themeswitcher;
use sauron::prelude::*;

pub fn err_box<T>(msg: &str) -> Node<T> {
    div(vec![class("error-box")], vec![text(msg)])
}
