use maud::{html, Markup};

use crate::layout::main_template;
use crate::util::{script, Document};

pub fn page() -> Markup {
    let main = html! {
        main {
            div id="terminal" {}
        }
        (script!{"
            const term = new Terminal();
            term.open(me('#terminal'));

            const fitAddon = new FitAddon.FitAddon();
            term.loadAddon(fitAddon);
            fitAddon.fit();

            const origin = location.origin.replace('http', 'ws');
            const socket = new WebSocket(`${origin}/api/terminal`);
            const attachAddon = new AttachAddon.AttachAddon(socket);
            term.loadAddon(attachAddon);

            const sendSize = () => socket.send(`size${term.cols},${term.rows}`);

            // Set initial terminal size once connected
            socket.addEventListener('open', sendSize);

            addEventListener('resize', () => fitAddon.fit());

            // Terminal onResize event only fires when the actual number of columns/rows changes
            term.onResize(sendSize);
        "})
    };

    let document = Document::new(main)
        .with_css(include_str!("terminal.css"))
        .with_css_links(&["xterm.css"])
        .with_script_links(&["xterm.js", "xterm-addon-attach.js", "xterm-addon-fit.js"]);
    main_template(&document)
}
