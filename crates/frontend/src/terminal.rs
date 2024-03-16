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
            "/static/xterm.js",
            "/static/xterm-addon-attach.js",
            "/static/terminal.js",
        ]),
        addl_css: Some(&["/static/xterm.css"]),
    })
}
