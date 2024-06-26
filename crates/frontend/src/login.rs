use axum::extract::RawQuery;
use maud::{html, Markup};

use crate::layout;

pub async fn page(RawQuery(query): RawQuery) -> Markup {
    let incorrect = query.is_some_and(|x| x.contains("incorrect"));

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

    layout::main_template(&main.into())
}
