use super::shared;
use axum::extract::State;
use maud::{html, Markup, DOCTYPE};
use pretty_bytes_typed::pretty_bytes_binary;
use tokio::sync::oneshot;

use crate::sysdata::{Request, RequestTx};

pub async fn system_page() -> Markup {
    html! {
        (DOCTYPE)

        head {
            link rel="stylesheet" href="/vendored/open-props.css";
            link rel="stylesheet" href="/vendored/index.css";
        }

        body {
            (shared::nav_menu())

            (shared::header())

            main hx-get="/api/system" hx-trigger="every 2s, load" {}

            (shared::footer())

            script src="/vendored/htmx.js" {}
        }
    }
}

pub async fn system_api(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::System(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    let pretty_ram_used = pretty_bytes_binary(resp.ram.used, Some(2));
    let pretty_ram_total = pretty_bytes_binary(resp.ram.total, Some(2));

    html! {
        section {
            header {
                "System Statistics"
            }

            "CPU usage: " (resp.cpu) "%"
            div .meter-container {
                div #cpu-meter style={"width:" (resp.cpu) "%"} {}
            }
            br;
            "RAM usage: " (pretty_ram_used) " / " (pretty_ram_total)
            div .meter-container {
                div #ram-meter style={"width:" (resp.ram.percent) "%"} {}
            }
        }
    }
}
