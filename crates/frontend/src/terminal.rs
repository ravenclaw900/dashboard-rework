use maud::{html, Markup};

use crate::layout::{self, Document};

pub async fn terminal_page() -> Markup {
    let main = html! {
        main {
            div id="terminal" {}
        }
    };

    layout::main_template(&Document {
        markup: main,
        addl_scripts: Some(&[
            "xterm.js",
            "xterm-addon-attach.js",
            "xterm-addon-fit.js",
            "terminal.js",
        ]),
        addl_css: Some(&["xterm.css"]),
    })
}
