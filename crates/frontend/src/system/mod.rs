use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use sysdata::{Request, RequestTx};

use crate::layout::main_template;
use crate::util::{send_req, Document};

#[tracing::instrument(name = "system_page", skip_all)]
pub async fn page(tx: RequestTx) -> Markup {
    let main = html! {
        main {
            section {
                h2 {
                    "System Statistics"
                }
                div ajxl-path="/system/fragment" ajxl-event=":load :finish" ajxl-debounce="2000" {
                    (fragment(tx).await)
                }
            }
        }
    };

    let document = Document::new(main).with_css(include_str!("system.css"));
    main_template(&document)
}

pub async fn fragment(tx: RequestTx) -> Markup {
    let data = send_req!(Request::System, tx);

    let pretty_ram_used = pretty_bytes_binary(data.ram.used, Some(2));
    let pretty_ram_total = pretty_bytes_binary(data.ram.total, Some(2));

    let pretty_swap_used = pretty_bytes_binary(data.swap.used, Some(2));
    let pretty_swap_total = pretty_bytes_binary(data.swap.total, Some(2));

    html! {
        "CPU usage: " (data.cpu) "%"
        div .meter-container {
            div #cpu-meter style={"width:" (data.cpu) "%"} {}
        }
        br;
        "RAM usage: " (pretty_ram_used) " / " (pretty_ram_total)
        div .meter-container {
            div #ram-meter style={"width:" (data.ram.percent) "%"} {}
        }
        @if data.swap.total > 0 {
            br;
            "Swap usage: " (pretty_swap_used) " / " (pretty_swap_total)
            div .meter-container {
                div #swap-meter style={"width:" (data.swap.percent) "%"} {}
            }
        }
    }
}
