use humantime::format_duration;
use hyper_ext::{IncomingReq, UriExt};
use maud::{html, Markup};
use pretty_bytes_typed::pretty_bytes_binary;
use serde::Deserialize;
use sysdata::{Request, RequestTx};

use crate::layout::main_template;
use crate::util::{icon, send_req, Document};

#[derive(Deserialize)]
pub struct ProcessQuery {
    #[serde(default)]
    sort: Column,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
enum Column {
    #[default]
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

pub async fn page(req: IncomingReq, tx: RequestTx) -> Markup {
    let main = html! {
        main {
            section {
                h2 {
                    "Processes"
                }
                (table(req, tx).await)
            }
        }
    };

    let document = Document::new(main).with_css(include_str!("process.css"));
    main_template(&document)
}

pub async fn table(req: IncomingReq, tx: RequestTx) -> Markup {
    // Since default is provided, this can't fail
    let query: ProcessQuery = req.uri().deserialize_query().unwrap();
    let sort = query.sort;

    let headers = [
        ("PID", Column::Pid),
        ("Name", Column::Name),
        ("Status", Column::Status),
        ("CPU Usage", Column::Cpu),
        ("Memory Usage", Column::Mem),
        ("Runtime", Column::Runtime),
    ];

    html! {
        table #process-table {
            thead {
                tr ajxl-target="#process-table" ajxl-swap="idiomorphOuter" {
                    @for header in headers {
                        th {
                            button ajxl-path={"/process/table?sort=" (header.1.as_str())} {
                                // Space to add some space between header and sort icon
                                (header.0) " "
                                @if sort == header.1 {
                                    (icon!("fa6-solid:sort"))
                                }
                            }
                        }
                    }
                    th {
                        "Actions"
                    }
                }
            }
            tbody ajxl-path={"/process/tbody?sort=" (sort.as_str())} ajxl-event=":load :finish" ajxl-debounce="2000" {
                (tbody(req, tx).await)
            }
        }
    }
}

// Clippy seems to get confused by the macro
#[allow(clippy::branches_sharing_code)]
pub async fn tbody(req: IncomingReq, tx: RequestTx) -> Markup {
    // Since default is provided, this can't fail
    let query: ProcessQuery = req.uri().deserialize_query().unwrap();
    let sort = query.sort;

    let mut data = send_req!(Request::Process, tx);

    match sort {
        Column::Pid => data.sort_unstable_by(|a, b| a.pid.cmp(&b.pid)),
        Column::Name => data.sort_unstable_by(|a, b| a.name.cmp(&b.name)),
        Column::Status => data.sort_unstable_by(|a, b| a.status.cmp(&b.status)),
        Column::Cpu => data.sort_unstable_by(|a, b| a.cpu.total_cmp(&b.cpu)),
        Column::Mem => data.sort_unstable_by(|a, b| a.mem.cmp(&b.mem)),
        Column::Runtime => data.sort_unstable_by(|a, b| a.runtime.cmp(&b.runtime)),
    }

    html! {
        @for proc in data {
            tr #(proc.pid) {
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
                    @let pretty_runtime = format_duration(std::time::Duration::from_secs(proc.runtime));
                    (pretty_runtime)
                }
                td {
                    button title="Terminate" ajxl-path={"/api/process?signal=term&pid=" (proc.pid)} ajxl-method="post"
                    ajxl-swap="none" ajxl-event="click" {
                        (icon!("fa6-solid:ban"))
                    }
                    button title="Kill" ajxl-path={"/api/process?signal=kill&pid=" (proc.pid)} ajxl-method="post"
                    ajxl-swap="none" ajxl-event="click" {
                        (icon!("fa6-solid:skull"))
                    }
                    @if proc.status == "Stopped" {
                        button title="Resume" ajxl-path={"/api/process?signal=resume&pid=" (proc.pid)} ajxl-method="post"
                        ajxl-swap="none" ajxl-event="click" {
                            (icon!("fa6-solid:play"))
                        }
                    } @else {
                        button title="Stop" ajxl-path={"/api/process?signal=stop&pid=" (proc.pid)} ajxl-method="post"
                        ajxl-swap="none" ajxl-event="click" {
                            (icon!("fa6-solid:pause"))
                        }
                    }
                }
            }
        }
    }
}
