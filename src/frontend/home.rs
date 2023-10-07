use axum::extract::State;
use maud::{html, Markup, DOCTYPE};
use tokio::sync::oneshot;

use crate::sysdata::{Request, RequestTx};

pub async fn main_page() -> Markup {
    html! {
        (DOCTYPE)

        div hx-get="/api/system" hx-trigger="load, every 2s" {}

        script src="/vendored/htmx.js" {}
    }
}

pub async fn system_data(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::System(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    html! {
        "CPU usage: " (resp.cpu) "%"
        br;
        "RAM usage: " (resp.ram.used) " / " (resp.ram.total)
    }
}
