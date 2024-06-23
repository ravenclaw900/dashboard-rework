use maud::{html, Markup};

use crate::layout::{self, Document};

pub fn page() -> Markup {
    let main = html! {
        main {
            div id="terminal" {}
        }
    };

    layout::main_template(&Document {
        markup: main,
        addl_scripts: &[
            "xterm.js",
            "xterm-addon-attach.js",
            "xterm-addon-fit.js",
            "terminal.js",
        ],
        addl_css: &["xterm.css"],
    })
}
