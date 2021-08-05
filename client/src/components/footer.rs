use crate::nice;
use crate::state::AppState;
use sauron::prelude::*;

pub fn render<T>(state: &AppState) -> Node<T> {
    node! {
        <footer>
            <div class="inner">
                <div class="copyright">
                    <span class="mdiv">"(c) 2021 Enormous Cloud"</span>
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
