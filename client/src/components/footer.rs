use sauron::prelude::*;

pub fn render<T>() -> Node<T> {
    node! {
        <footer>
            <div class="inner">
                <div class="copyright">
                    "© 2021 Enormous Cloud | "
                    <a 
                        target="_blank"
                        href="https://github.com/EnormousCloud/api3-dao-tracker"
                    >
                        "Source"
                    </a>
                </div>
            </div>
        </footer>
    }
}
