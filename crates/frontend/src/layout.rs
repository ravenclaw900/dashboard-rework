use maud::{html, Markup, PreEscaped, DOCTYPE};

// Why did I put this macro here? Mostly because this module is already imported by all of the others.
macro_rules! send_req {
    ($req:path, $chan:ident) => {
        'a: {
            use hyper_ext::ErrorResponse;

            let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
            let send_req = $chan.send($req(resp_tx)).await;

            if send_req.is_err() {
                break 'a Err(ErrorResponse::new_server_err(ErrorResponse::CHANNEL_MSG));
            }

            resp_rx
                .await
                .map_err(|_| ErrorResponse::new_server_err(ErrorResponse::CHANNEL_MSG))
        }
    };
}

pub(crate) use send_req;

pub struct Document {
    pub markup: Markup,
    pub addl_css: &'static [&'static str],
    pub addl_scripts: &'static [&'static str],
}

impl From<Markup> for Document {
    fn from(value: Markup) -> Self {
        Self {
            markup: value,
            addl_css: &[],
            addl_scripts: &[],
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

                @for css in doc.addl_css {
                    link rel="stylesheet" href={"/static/" (css)};
                }

                script defer src="/static/htmx.js" {}

                @for script in doc.addl_scripts {
                    script defer src={"/static/" (script)} {}
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
    html! {
        footer {
            "DietPi Dashboard v"(config::VERSION)" by ravenclaw900"
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
