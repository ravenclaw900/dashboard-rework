use axum::extract::State;
use humantime::format_duration;
use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use sysdata::{Request, RequestTx};
use tokio::sync::oneshot;

use super::layout;

pub async fn process_page() -> Markup {
    layout::main_template(html! {
        main hx-get="/api/process" hx-trigger="load, every 2s" {}
    })
}

pub async fn process_api(State(tx): State<RequestTx>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::Process(resp_tx)).await.unwrap();

    let resp = resp_rx.await.unwrap();

    html! {
        section {
            header {
                "Processes"
            }
            table {
                thead {
                    tr {
                        th {
                            "PID"
                        }
                        th {
                            "Name"
                        }
                        th {
                            "Status"
                        }
                        th {
                            "CPU usage"
                        }
                        th {
                            "Memory usage"
                        }
                        th {
                            "Runtime"
                        }
                    }
                }
                @for proc in resp {
                    tr {
                        td {
                            (proc.pid)
                        }
                        td {
                            (proc.name)
                        }
                        td {
                            (proc.status)
                        }
                        td {
                            (proc.cpu)"%"
                        }
                        td {
                            @let pretty_memory = pretty_bytes_binary(proc.mem, Some(2));
                            (pretty_memory)
                        }
                        td {
                            @let pretty_runtime = format_duration(proc.runtime);
                            (pretty_runtime)
                        }
                    }
                }
            }
        }
    }
}
