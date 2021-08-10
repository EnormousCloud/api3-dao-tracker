use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;
use web3::types::{H160, H256};

pub fn link_eventlog<T>(chain_id: u64, block_number: u64, tx: H256) -> Node<T> {
    let link = match chain_id {
        1 => Some(format!("https://etherscan.io/tx/{:?}#eventlog", tx)),
        4 => Some(format!("https://rinkeby.etherscan.io/tx/{:?}#eventlog", tx)),
        _ => None,
    };
    match link {
        Some(link) => node! {
            <a href={link} rel="nofollow noopener noreferrer" target="_blank">
                {text(nice::int(block_number))}
            </a>
        },
        None => text(nice::int(block_number)),
    }
}

pub fn link<T>(link: String) -> Node<T> {
    if link.len() == 0 {
        return text("");
    }
    node! {
        <a href={link.clone()} rel="nofollow noopener noreferrer" target="_blank">
            {text(link.clone())}
        </a>
    }
}

pub fn link_wallet<T>(state: &AppState, addr: H160) -> Node<T> {
    match state.wallets.get(&addr) {
        Some(w) => {
            let labels = state.get_labels(w);
            node! {
                <a href={format!("wallets/{:?}", w.address)}>
                    <div>
                        {span(vec![class("badges")], labels.iter().map(|v| {
                            let title = format!("{}", v.title);
                            node! {
                                <span class={format!("badge {}", v.class)} title={title}>{text(v.text.clone().as_str())}</span>
                            }
                        }).collect::<Vec<Node<T>>>())}
                        {match &w.ens {
                            Some(ens) => strong(vec![class("ens")],vec![text(ens)]),
                            None => span(vec![],vec![]),
                        }}
                    </div>
                    <div>{text(format!("{:?}", w.address))}</div>
                </a>
            }
        }
        None => span(vec![], vec![text(format!("{:?}", addr))]),
    }
}
