use axum::extract::{Query, State};
use humantime::format_duration;
use maud::{html, Markup, PreEscaped};
use pretty_bytes_typed::pretty_bytes_binary;
use serde::Deserialize;
use sysdata::{Request, RequestTx};
use tokio::sync::oneshot;

use super::layout;

#[derive(Deserialize)]
pub struct ProcessQuery {
    sort: Column,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Column {
    Pid,
    Name,
    Status,
    Cpu,
    Mem,
    Runtime,
}

impl Column {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Pid => "pid",
            Self::Name => "name",
            Self::Status => "status",
            Self::Cpu => "cpu",
            Self::Mem => "mem",
            Self::Runtime => "runtime",
        }
    }
}

pub async fn process_page() -> Markup {
    let main = html! {
        main {
            section {
                header {
                    "Processes"
                }
                table hx-get="/api/process?sort=pid" hx-trigger="load" hx-swap="outerHTML" {}
            }
        }
    };
    layout::main_template(&main)
}

// Clippy seems to get confused by the macro
#[allow(clippy::branches_sharing_code)]
pub async fn process_api(State(tx): State<RequestTx>, Query(query): Query<ProcessQuery>) -> Markup {
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request::Process(resp_tx)).await.unwrap();

    let mut resp = resp_rx.await.unwrap();

    let sort = query.sort;

    resp.sort_unstable_by(|a, b| {
        match sort {
            Column::Pid => a.pid.cmp(&b.pid),
            Column::Name => a.name.cmp(&b.name),
            Column::Status => a.status.cmp(&b.status),
            // Guaranteed to not be NaN, so unwrap is safe here
            Column::Cpu => a.cpu.partial_cmp(&b.cpu).unwrap(),
            Column::Mem => a.mem.cmp(&b.mem),
            Column::Runtime => a.runtime.cmp(&b.runtime),
        }
    });

    let headers = [
        ("PID", Column::Pid),
        ("Name", Column::Name),
        ("Status", Column::Status),
        ("CPU Usage", Column::Cpu),
        ("Memory Usage", Column::Mem),
        ("Runtime", Column::Runtime),
    ];

    html! {
        // Use 'load polling' technique
        table hx-get={"/api/process?sort=" (sort.as_str())} hx-trigger="load delay:2s" hx-swap="outerHTML" hx-target="this" {
            thead {
                tr {
                    @for header in headers {
                        th {
                            button hx-get={"/api/process?sort=" (header.1.as_str())} {
                                // Space to add some space between header and sort icon
                                (header.0) " "
                                @if sort == header.1 {
                                    (PreEscaped(iconify::svg!("fa6-solid:sort")))
                                }
                            }
                        }
                    }
                    th {
                        "Actions"
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
                    td ."actions-cell" {
                        button title="Terminate" hx-post={"/api/process/" (proc.pid) "?signal=term"} hx-swap="none" {
                            (PreEscaped(iconify::svg!("fa6-solid:ban")))
                        }
                        button title="Kill" hx-post={"/api/process/" (proc.pid) "?signal=kill"} hx-swap="none" {
                            (PreEscaped(iconify::svg!("fa6-solid:skull")))
                        }
                        @if proc.status == "Stopped" {
                            button title="Resume" hx-post={"/api/process/" (proc.pid) "?signal=resume"} hx-swap="none" {
                                (PreEscaped(iconify::svg!("fa6-solid:play")))
                            }
                        } @else {
                            button title="Stop" hx-post={"/api/process/" (proc.pid) "?signal=stop"} hx-swap="none" {
                                (PreEscaped(iconify::svg!("fa6-solid:pause")))
                            }
                        }
                    }
                }
            }
        }
    }
}
