use axum::extract::{Query, State};
use humantime::format_duration;
use maud::{html, Markup, PreEscaped};
use pretty_bytes_typed::pretty_bytes_binary;
use serde::Deserialize;
use sysdata::{types::ProcessData, Request, RequestTx};

use crate::layout::{main_template, send_req, ChannelSendError};

#[derive(Deserialize)]
pub struct ProcessQuery {
    sort: Column,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
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
    const fn as_str(self) -> &'static str {
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

#[tracing::instrument(name = "process_page", skip_all, err)]
pub async fn page(State(tx): State<RequestTx>) -> Result<Markup, ChannelSendError> {
    let mut data = send_req!(Request::Process, tx)?;

    let main = html! {
        main {
            section {
                h2 {
                    "Processes"
                }
                (inner(&mut data, Column::Pid))
            }
        }
    };

    Ok(main_template(&main.into()))
}

#[tracing::instrument(name = "process_fragment", skip_all, err)]
pub async fn fragment(
    State(tx): State<RequestTx>,
    Query(query): Query<ProcessQuery>,
) -> Result<Markup, ChannelSendError> {
    let mut data = send_req!(Request::Process, tx)?;

    Ok(inner(&mut data, query.sort))
}

// Clippy seems to get confused by the macro
#[allow(clippy::branches_sharing_code)]
fn inner(data: &mut [ProcessData], sort: Column) -> Markup {
    match sort {
        Column::Pid => data.sort_unstable_by(|a, b| a.pid.cmp(&b.pid)),
        Column::Name => data.sort_unstable_by(|a, b| a.name.cmp(&b.name)),
        Column::Status => data.sort_unstable_by(|a, b| a.status.cmp(&b.status)),
        Column::Cpu => data.sort_unstable_by(|a, b| a.cpu.total_cmp(&b.cpu)),
        Column::Mem => data.sort_unstable_by(|a, b| a.mem.cmp(&b.mem)),
        Column::Runtime => data.sort_unstable_by(|a, b| a.runtime.cmp(&b.runtime)),
    }

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
        table hx-get={"/process/htmx?sort=" (sort.as_str())} hx-trigger="load delay:2s" hx-swap="outerHTML" hx-target="this" {
            tr {
                @for header in headers {
                    th {
                        button hx-get={"/process/htmx?sort=" (header.1.as_str())} {
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
            @for proc in data {
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
                    td {
                        div ."actions-cell" {
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
}
