use maud::{html, Markup, DOCTYPE};

pub fn main_template(endpoint: &str) -> Markup {
    html! {
        (DOCTYPE)

        head {
            link rel="stylesheet" href="/static/vars.css";
            link rel="stylesheet" href="/static/index.css";
        }

        body {
            (nav_menu())

            (header())

            main hx-get=(endpoint) hx-trigger="load" hx-swap="outerHTML" {}

            (footer())

            script src="/static/htmx.js" {}
            script src="/static/iconify.js" {}
        }
    }
}

fn header() -> Markup {
    html! {
        header {
            "DietPi Dashboard"
        }
    }
}

fn footer() -> Markup {
    let current_version = env!("CARGO_PKG_VERSION");

    html! {
        footer {
            "DietPi Dashboard v"(current_version)" by ravenclaw900"
            a href="https://github.com/ravenclaw900/dashboard-rework" target="_blank" {
                iconify-icon icon="cib:github" style="font-size:var(--font-size-4);color:black;" {}
            }
        }
    }
}

fn nav_menu() -> Markup {
    html! {
        nav {
            div {
                "DietPi Dashboard"
            }
            ul {
                a href="/system" {
                    iconify-icon icon="fa6-solid:database" {}
                    "System"
                }
                a href="/process" {
                    iconify-icon icon="fa6-solid:microchip" {}
                    "Processes"
                }
            }
        }
    }
}
