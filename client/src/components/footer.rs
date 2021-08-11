use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;

pub fn render<T>(state: &AppState) -> Node<T> {
    let testnet: Node<T> = if state.chain_id == 4 {
        span(vec![class("mdiv badge badge-testnet")], vec![text("rinkeby")]) 
    } else if state.chain_id != 1 {
        span(vec![class("mdiv badge badge-testnet")], vec![text("testnet")]) 
    } else {
        span(vec![],vec![])
    };
    let footer_class= if state.chain_id == 1 { "" }  else { "testnet" };
    node! {
        <footer class={footer_class}>
            <div class="inner">
                <div class="copyright">
                    {testnet} " "
                    <span class="mdiv">" © 2021 Enormous Cloud "</span>
                    <span class="desktop-only">" | "</span>
                    <span class="mdiv">
                        <a
                            target="_blank"
                            href="https://github.com/EnormousCloud/api3-dao-tracker"
                        >
                            "Github Source"
                        </a>
                    </span>
                    <span class="desktop-only">" | "</span>
                    <span class="mdiv">
                        { text("Last block: ")}
                        { text(nice::int(state.last_block)) }
                    </span>
                </div>
            </div>
        </footer>
    }
}
