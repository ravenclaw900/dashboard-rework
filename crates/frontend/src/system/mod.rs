use hyper_ext::ErrorResponse;
use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use sysdata::{types::SystemData, Request, RequestTx};

use crate::layout::{main_template};
use crate::util::{Document, send_req};

#[tracing::instrument(name = "system_page", skip_all, err)]
pub async fn page(tx: RequestTx) -> Result<Markup, ErrorResponse> {
    let data = send_req!(Request::System, tx)?;

    let main = html! {
        main {
            section {
                h2 {
                    "System Statistics"
                }
                div hx-get="/system/htmx" hx-trigger="every 2s" {
                    (inner(&data))
                }
            }
        }
    };

    let document = Document::new(main).with_css(include_str!("system.css"));
    Ok(main_template(&document))
}

#[tracing::instrument(name = "system_fragment", skip_all, err)]
pub async fn fragment(tx: RequestTx) -> Result<Markup, ErrorResponse> {
    let data = send_req!(Request::System, tx)?;

    Ok(inner(&data))
}

fn inner(data: &SystemData) -> Markup {
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
