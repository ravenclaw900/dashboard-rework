use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn main_template(main: &Markup) -> Markup {
    html! {
        (DOCTYPE)

        head {
            link rel="stylesheet" href="/static/vars.css";
            link rel="stylesheet" href="/static/index.css";
        }

        body {
            (nav_menu())

            (header())

            (main)

            (footer())

            script src="/static/htmx.js" {}
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
                (PreEscaped(iconify::svg!("cib:github", width="32", color="black")))
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
                a href="/login" {
                    (PreEscaped(iconify::svg!("fa6-solid:arrow-right-to-bracket")))
                    "Login"
                }
                a href="/system" {
                    (PreEscaped(iconify::svg!("fa6-solid:database")))
                    "System"
                }
                a href="/process" {
                    (PreEscaped(iconify::svg!("fa6-solid:microchip")))
                    "Processes"
                }
            }
        }
    }
}
