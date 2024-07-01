use maud::{html, Markup};

use crate::layout::main_template;
use crate::util::Document;

pub fn page() -> Markup {
    let main = html! {
        main {
            div id="terminal" {}
        }
    };

    let document = Document::new(main)
        .with_css(include_str!("terminal.css"))
        .with_script(include_str!("terminal.js"))
        .with_css_links(&["xterm.css"])
        .with_script_links(&["xterm.js", "xterm-addon-attach.js", "xterm-addon-fit.js"]);
    main_template(&document)
}
