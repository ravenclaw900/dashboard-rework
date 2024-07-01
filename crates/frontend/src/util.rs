use maud::Markup;

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
    pub css: Option<&'static str>,
    pub script: Option<&'static str>,
    pub css_links: &'static [&'static str],
    pub script_links: &'static [&'static str],
}

impl Document {
    pub fn new(markup: Markup) -> Self {
        Self {
            markup,
            css: None,
            script: None,
            css_links: &[],
            script_links: &[],
        }
    }

    pub fn with_css(mut self, css: &'static str) -> Self {
        self.css = Some(css);
        self
    }

    pub fn with_script(mut self, script: &'static str) -> Self {
        self.script = Some(script);
        self
    }

    pub fn with_css_links(mut self, css_links: &'static [&'static str]) -> Self {
        self.css_links = css_links;
        self
    }

    pub fn with_script_links(mut self, script_links: &'static [&'static str]) -> Self {
        self.script_links = script_links;
        self
    }
}
