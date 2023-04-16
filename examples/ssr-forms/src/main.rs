#![deny(warnings)]
use chrono::Local;
use sauron::{html::attributes, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::{http::Response, Filter};

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    name: String,
    biography: String,
    thought: Option<String>,
}

fn view(form_data: Option<FormData>) -> Node<()> {
    println!("in view  with form_data: {:#?}", form_data);
    let date_str: String = Local::now().to_string();
    form(
        [
            action("/submission"),
            attributes::method("GET"),
            class("some-class"),
            id("some-id"),
            attr("form_data-id", 1),
        ],
        [
            div(
                [id("current-time")],
                [text(format!("Today is {}", date_str))],
            ),
            div(
                [],
                [
                    text("Your name is: "),
                    input(
                        [
                            attributes::name("name"),
                            r#type("text"),
                            placeholder("John Smith"),
                        ],
                        [],
                    ),
                ],
            ),
            if let Some(form_data) = &form_data {
                p([], [text(format!("Hello {}!", form_data.name))])
            } else {
                text("")
            },
            div(
                [],
                [
                    p([], [text("Tell us something about yourself:")]),
                    div(
                        [],
                        [textarea(
                            [
                                attributes::name("biography"),
                                rows(10),
                                cols(80),
                                placeholder("I'm a..."),
                            ],
                            [],
                        )],
                    ),
                    if let Some(form_data) = &form_data {
                        p([], [text(format!("{}", form_data.biography))])
                    } else {
                        text("")
                    },
                ],
            ),
            div(
                [],
                [
                    text("What are you thinking right now?"),
                    input(
                        [
                            attributes::name("thought"),
                            r#type("text"),
                            placeholder("Elephants..."),
                        ],
                        [],
                    ),
                    if let Some(form_data) = &form_data {
                        if let Some(thought) = &form_data.thought {
                            text(format!("Hmmn {}... Interesting.", thought))
                        } else {
                            text("")
                        }
                    } else {
                        text("")
                    },
                ],
            ),
            input([r#type("submit"), value("Submit")], []),
        ],
    )
}

#[tokio::main]
async fn main() {
    let submission = warp::get()
        .and(warp::path("submission"))
        .and(warp::query::<FormData>())
        .map(|form_data: FormData| {
            println!("form data: {:#?}", form_data);
            let mut buffer = String::new();
            let node = view(Some(form_data));
            node.render(&mut buffer).expect("must render");
            Response::builder().body(buffer)
        });

    let index = warp::get().map(|| {
        let mut buffer = String::new();
        let node = view(None);
        node.render(&mut buffer).expect("must render");
        Response::builder().body(buffer)
    });

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(submission.or(index)).run(socket).await;
}
