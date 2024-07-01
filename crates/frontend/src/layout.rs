use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::util::Document;

pub fn main_template(doc: &Document) -> Markup {
    html! {
        (DOCTYPE)

        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                link rel="icon" href="/static/favicon.png";

                link rel="stylesheet" href="/static/vars.css";
                link rel="stylesheet" href="/static/main.css";

                @for link in doc.css_links {
                    link rel="stylesheet" href={"/static/" (link)};
                }

                @if let Some(css) = doc.css {
                    style { (PreEscaped(css)) }
                }

                script defer src="/static/htmx.js" {}
            }

            body {
                (nav_menu())

                (header())

                (doc.markup)

                (footer())

                @for link in doc.script_links {
                    script src={"/static/" (link)} {}
                }

                @if let Some(script) = doc.script {
                    script { (PreEscaped(script)) }
                }
            }
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
    html! {
        footer {
            "DietPi Dashboard v"(config::VERSION)" by ravenclaw900"
            a href="https://github.com/ravenclaw900/dashboard-rework" target="_blank" {
                (PreEscaped(iconify::svg!("cib:github", width="28", color="black")))
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
                    (PreEscaped(iconify::svg!("fa6-solid:database")))
                    "System"
                }
                a href="/process" {
                    (PreEscaped(iconify::svg!("fa6-solid:microchip")))
                    "Processes"
                }
                a href="/management" {
                    (PreEscaped(iconify::svg!("fa6-solid:user")))
                    "Management"
                }
                a href="/terminal" {
                    (PreEscaped(iconify::svg!("fa6-solid:terminal")))
                    "Terminal"
                }
            }
        }
    }
}
