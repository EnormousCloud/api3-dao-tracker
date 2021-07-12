use sauron::prelude::*;

pub fn render<T>() -> Node<T> {
    node! {
        <footer>
            <div class="inner">
                <div class="copyright">
                    "© 2021 Enormous Cloud"
                </div>
            </div>
        </footer>
    }
}
