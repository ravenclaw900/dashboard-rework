use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        header {
            "DietPi Dashboard"
        }
    }
}

pub fn footer() -> Markup {
    let current_version = env!("CARGO_PKG_VERSION");

    html! {
        footer {
            "DietPi Dashboard v"(current_version)" by ravenclaw900"
        }
    }
}

pub fn nav_menu() -> Markup {
    html! {
        nav {
            div {
                "DietPi Dashboard"
            }
            ul {
                a href="https://example.com" {
                    "Test"
                }
            }
        }
    }
}
