use sauron::prelude::*;

pub fn render<T>(title: &str, divclass: &'static str, content: Node<T>) -> Node<T> {
    div(
        vec![class(divclass)],
        vec![node! {
            <div class="bordered-wrapper">
                <div class="bordered-panel">
                    <div class="bordered-box">
                        <div class="bordered-left"></div>
                        <div class="bordered-inner">
                            <div class="bordered-title big-title">{text(title)}</div>
                            <div class="bordered-content">{ content }</div>
                        </div>
                        <div class="bordered-right"></div>
                    </div>
                </div>
            </div>
        }],
    )
}
