use crate::state::AppState;
use sauron::prelude::*;

pub struct MenuItem {
    is_active: bool,
    href: &'static str,
    title: &'static str,
}

impl MenuItem {
    pub fn render<T>(&self) -> Node<T> {
        let mut attr = vec![href(self.href), class("menu-item")];
        if self.is_active {
            attr.push(class("active"));
        }
        a(attr, vec![text(self.title)])
    }
}

const TITLE: &'static str = "API3 DAO Tracker";
const SLOGAN: &'static str = "on-chain analytics: members, staking rewards, API3 token supply";

pub fn render<T>(active_menu: &'static str, state: &AppState) -> Node<T> {
    let is_default = !active_menu.starts_with("/rewards")
        && !active_menu.starts_with("/wallets")
        && !active_menu.starts_with("/votings")
        && !active_menu.starts_with("/treasury");

    let menu: Vec<MenuItem> = vec![
        MenuItem {
            href: "./",
            title: "API3 DAO",
            is_active: is_default,
        },
        MenuItem {
            href: "./rewards",
            title: "Rewards",
            is_active: active_menu.starts_with("/rewards"),
        },
        MenuItem {
            href: "./wallets",
            title: "Wallets",
            is_active: active_menu.starts_with("/wallets"),
        },
        MenuItem {
            href: "./votings",
            title: "Votings",
            is_active: active_menu.starts_with("/votings"),
        },
        MenuItem {
            href: "./treasury",
            title: "Treasury",
            is_active: active_menu.starts_with("/treasury"),
        },
    ];

    let testnet: Node<T> = if state.chain_id == 4 {
        span(vec![class("badge badge-testnet")], vec![text("rinkeby")])
    } else if state.chain_id != 1 {
        span(vec![class("badge badge-testnet")], vec![text("testnet")])
    } else {
        span(vec![], vec![])
    };

    let header_class = if state.chain_id == 1 { "" } else { "testnet" };
    node! {
      <header class={header_class}>
        <div class="inner">
          <div class="nav-brand">
            <span class="nav-brand__label">
              {text(TITLE)}
            </span>
            <span class="nav-brand__slogan">
              {testnet} {text(SLOGAN)}
            </span>
          </div>
          <div class="mid"></div>
          {
            div(
              vec![class("desktop-menu")],
              menu.iter().map(|x| x.render()).collect::<Vec<Node<T>>>(),
            )
          }
        </div>
      </header>
    }
}
