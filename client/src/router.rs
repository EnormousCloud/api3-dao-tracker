use crate::nice;
use sauron::prelude::*;
use web3::types::H256;

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