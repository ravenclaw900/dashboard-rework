use std::time::Duration;

use humantime::format_duration;
use maud::{html, Markup};
use sysdata::{Request, RequestTx};

use crate::layout::{main_template, send_req, ChannelSendError};

#[tracing::instrument(name = "management_page", skip_all, err)]
pub async fn page(tx: RequestTx) -> Result<Markup, ChannelSendError> {
    let (data, uptime) = send_req!(Request::Host, tx)?;

    let pretty_uptime = format_duration(Duration::from_secs(uptime));

    let rows = [
        ("Hostname:", data.hostname),
        ("Uptime:", pretty_uptime.to_string()),
        ("Network Interface:", data.net_interface),
        ("IP Address:", data.ip_addr),
        ("OS Version:", data.system_version),
        ("DietPi Version:", data.dietpi_version),
        (
            "Installed Packages:",
            format!(
                "{} ({} upgradable)",
                data.installed_packages, data.upgradable_packages
            ),
        ),
    ];

    let main = html! {
        main {
            section {
                h2 {
                    "System Information"
                }
                table #management-table {
                    @for row in rows {
                        tr {
                            td {
                                (row.0)
                            }
                            td {
                                (row.1)
                            }
                        }
                    }
                }
            }
            section {
                h2 {
                    "System Actions"
                }
                p {
                    "test"
                }
            }
        }
    };

    Ok(main_template(&main.into()))
}
