use maud::{html, Markup, PreEscaped, DOCTYPE};

pub struct Document {
    pub markup: Markup,
    pub addl_css: Option<&'static [&'static str]>,
    pub addl_scripts: Option<&'static [&'static str]>,
}

impl From<Markup> for Document {
    fn from(value: Markup) -> Self {
        Self {
            markup: value,
            addl_css: None,
            addl_scripts: None,
        }
    }
}

pub fn main_template(doc: &Document) -> Markup {
    html! {
        (DOCTYPE)

        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="/static/vars.css";
                link rel="stylesheet" href="/static/index.css";

                @if let Some(styles) = doc.addl_css {
                    @for css in styles {
                        link rel="stylesheet" href={"/static/" (css)};
                    }
                }

                script defer src="/static/htmx.js" {}

                @if let Some(scripts) = doc.addl_scripts {
                    @for script in scripts {
                        script defer src={"/static/" (script)} {}
                    }
                }
            }

            body {
                (nav_menu())

                (header())

                (doc.markup)

                (footer())
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
                a href="/system" {
                    (PreEscaped(iconify::svg!("fa6-solid:database")))
                    "System"
                }
                a href="/process" {
                    (PreEscaped(iconify::svg!("fa6-solid:microchip")))
                    "Processes"
                }
                a href="/terminal" {
                    (PreEscaped(iconify::svg!("fa6-solid:terminal")))
                    "Terminal"
                }
            }
        }
    }
}
