use hyper_ext::IncomingReq;
use maud::{html, Markup};

use crate::layout::main_template;
use crate::util::Document;

pub fn page(req: IncomingReq) -> Markup {
    let incorrect = req.uri().query().is_some_and(|x| x.contains("incorrect"));

    let main = html! {
        main {
            @if incorrect {
                "Incorrect Password"
            }
            form action="/api/login" method="post" {
                "Password: " input type="password" name="pass" {}
                br;
                input type="submit" {}
            }
        }
    };

    let document = Document::new(main);
    main_template(&document)
}
