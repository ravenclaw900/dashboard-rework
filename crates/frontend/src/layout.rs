use maud::{html, Markup, DOCTYPE};

pub fn main_template(content: Markup) -> Markup {
    html! {
        (DOCTYPE)

        head {
            link rel="stylesheet" href="/static/vars.css";
            link rel="stylesheet" href="/static/index.css";
        }

        body {
            (nav_menu())

            (header())

            (content)

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
                    "System"
                }
            }
        }
    }
}
