#![deny(warnings)]
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    Cmd,
    Component,
    Http,
    Node,
    Program,
};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    Click,
    ReceivedData(Result<Data, JsValue>),
}

pub struct App {
    click_count: u32,
    data: Data,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Data {
    page: i32,
    per_page: i32,
    total: i32,
    total_pages: i32,
    data: Vec<User>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
    avatar: String,
}

impl App {
    pub fn new() -> Self {
        App {
            click_count: 0,
            data: Data::default(),
        }
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        let url = "https://reqres.in/api/users";
        let data_decoder = |v: String| {
            let data: Result<Data, _> = serde_json::from_str(&v);
            sauron::log!("data: {:#?}", data);
            data.expect("Error deserializing data")
        };
        Http::fetch_with_text_response_decoder(
            url,
            data_decoder,
            Msg::ReceivedData,
        )
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                div(
                    vec![
                        class("some-class"),
                        id("some-id"),
                        attr("data-id", 1),
                    ],
                    vec![
                        input(
                            vec![
                                class("client"),
                                r#type("button"),
                                value("Click me!"),
                                onclick(|_| {
                                    sauron::log("Button is clicked");
                                    Msg::Click
                                }),
                            ],
                            vec![],
                        ),
                        text(format!("Clicked: {}", self.click_count)),
                    ],
                ),
                div(vec![], vec![]).add_children(
                    self.data
                        .data
                        .iter()
                        .map(|user| {
                            ul(
                                vec![],
                                vec![
                                    li(vec![], vec![text(&user.id)]),
                                    li(vec![], vec![text(&user.email)]),
                                    li(vec![], vec![text(&user.first_name)]),
                                    li(
                                        vec![],
                                        vec![img(
                                            vec![src(&user.avatar)],
                                            vec![],
                                        )],
                                    ),
                                ],
                            )
                        })
                        .collect(),
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        sauron::log!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::ReceivedData(Ok(data)) => {
                self.data = data;
            }
            Msg::ReceivedData(Err(js_value)) => {
                sauron::log!("Error fetching users! {:#?}", js_value);
            }
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let json = r#"
{"page":1,"per_page":3,"total":12,"total_pages":4,"data":[{"id":1,"email":"george.bluth@reqres.in","first_name":"George","last_name":"Bluth","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/calebogden/128.jpg"},{"id":2,"email":"janet.weaver@reqres.in","first_name":"Janet","last_name":"Weaver","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/josephstein/128.jpg"},{"id":3,"email":"emma.wong@reqres.in","first_name":"Emma","last_name":"Wong","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/olegpogodaev/128.jpg"}]}
        "#;
        println!("json: {}", json);
        let data: Result<Data, _> = serde_json::from_str(json);
        println!("data: {:#?}", data);
        assert!(data.is_ok());
    }
}
