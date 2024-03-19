use axum::extract::State;
use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use sysdata::{types::SystemData, Request, RequestTx};

use crate::layout::{main_template, send_req};

pub async fn system_page(State(tx): State<RequestTx>) -> Markup {
    let resp = send_req!(Request::System, tx);

    let main = html! {
        main {
            section {
                header {
                    "System Statistics"
                }
                div hx-get="/api/system" hx-trigger="every 2s" {
                    (system_inner(&resp))
                }
            }
        }
    };
    main_template(&main.into())
}

pub async fn system_api(State(tx): State<RequestTx>) -> Markup {
    let resp = send_req!(Request::System, tx);

    system_inner(&resp)
}

fn system_inner(data: &SystemData) -> Markup {
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
        br;
        "Swap usage: " (pretty_swap_used) " / " (pretty_swap_total)
        div .meter-container {
            div #swap-meter style={"width:" (data.swap.percent) "%"} {}
        }
    }
}
