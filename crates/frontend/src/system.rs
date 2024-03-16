use axum::extract::State;
use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use sysdata::{Request, RequestTx};
use tokio::sync::oneshot;

use crate::layout;

pub async fn system_page() -> Markup {
    let main = html! {
        main {
            section {
                header {
                    "System Statistics"
                }
                div hx-get="/api/system" hx-trigger="load, every 2s" {}
            }
        }
    };
    layout::main_template(&main.into())
}

pub async fn system_api(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::System(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    let pretty_ram_used = pretty_bytes_binary(resp.ram.used, Some(2));
    let pretty_ram_total = pretty_bytes_binary(resp.ram.total, Some(2));

    let pretty_swap_used = pretty_bytes_binary(resp.swap.used, Some(2));
    let pretty_swap_total = pretty_bytes_binary(resp.swap.total, Some(2));

    html! {
        "CPU usage: " (resp.cpu) "%"
        div .meter-container {
            div #cpu-meter style={"width:" (resp.cpu) "%"} {}
        }
        br;
        "RAM usage: " (pretty_ram_used) " / " (pretty_ram_total)
        div .meter-container {
            div #ram-meter style={"width:" (resp.ram.percent) "%"} {}
        }
        br;
        "Swap usage: " (pretty_swap_used) " / " (pretty_swap_total)
        div .meter-container {
            div #swap-meter style={"width:" (resp.swap.percent) "%"} {}
        }
    }
}
