use maud::Markup;

macro_rules! send_req {
    ($req:path, $chan:ident) => {{
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        $chan
            .send($req(resp_tx))
            .await
            .expect("failed to send sysdata request");

        resp_rx.await.expect("failed to recv sysdata request")
    }};
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
    pub const fn new(markup: Markup) -> Self {
        Self {
            markup,
            css: None,
            script: None,
            css_links: &[],
            script_links: &[],
        }
    }

    pub const fn with_css(mut self, css: &'static str) -> Self {
        self.css = Some(css);
        self
    }

    pub const fn with_script(mut self, script: &'static str) -> Self {
        self.script = Some(script);
        self
    }

    pub const fn with_css_links(mut self, css_links: &'static [&'static str]) -> Self {
        self.css_links = css_links;
        self
    }

    pub const fn with_script_links(mut self, script_links: &'static [&'static str]) -> Self {
        self.script_links = script_links;
        self
    }
}
