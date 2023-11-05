use super::shared;
use axum::extract::State;
use maud::{html, Markup, DOCTYPE};
use pretty_bytes_typed::pretty_bytes_binary;
use tokio::sync::oneshot;

use crate::{
    sysdata::{Request, RequestTx},
    types,
};

pub async fn system_page(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::System(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    html! {
        (DOCTYPE)

        head {
            link rel="stylesheet" href="/vendored/open-props.css";
            link rel="stylesheet" href="/vendored/index.css";
            script src="/vendored/css-scope-inline.js" {}
        }

        body {
            (shared::header())

            (system_html(resp))

            script src="/vendored/htmx.js" {}
        }
    }
}

pub async fn system_api(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::System(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    system_html(resp)
}

fn system_html(system_data: types::SystemData) -> Markup {
    let pretty_ram_used = pretty_bytes_binary(system_data.ram.used, Some(2));
    let pretty_ram_total = pretty_bytes_binary(system_data.ram.total, Some(2));

    html! {
        div hx-get="/api/system" hx-trigger="every 2s" hx-swap="outerHTML" {
            "CPU usage: " (system_data.cpu) "%"
            div .meter-container {
                div #cpu-meter style={"width:" (system_data.cpu) "%"} {}
            }
            br;
            "RAM usage: " (pretty_ram_used) " / " (pretty_ram_total)
            div .meter-container {
                div #ram-meter style={"width:" (system_data.ram.percent) "%"} {}
            }
            br;
        }
    }
}
