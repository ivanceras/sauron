#![deny(warnings)]
use sauron::js_sys::TypeError;
use sauron::prelude::*;
use serde::Deserialize;

#[macro_use]
extern crate log;

const DATA_URL: &'static str = "https://reqres.in/api/users";

#[derive(Debug)]
pub enum Msg {
    NextPage,
    PrevPage,
    ReceivedData(Data),
    JsonError(serde_json::Error),
    RequestError(TypeError),
}

pub struct App {
    page: u32,
    data: Data,
    error: Option<String>,
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
            page: 1,
            data: Data::default(),
            error: None,
        }
    }

    fn fetch_page(&self) -> Cmd<Self, Msg> {
        let url = format!("{}?page={}", DATA_URL, self.page);
        Http::fetch_with_text_response_decoder(
            &url,
            |v: String| match serde_json::from_str(&v) {
                Ok(data) => Msg::ReceivedData(data),
                Err(err) => Msg::JsonError(err),
            },
            Msg::RequestError,
        )
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        console_log::init_with_level(log::Level::Trace).unwrap();
        self.fetch_page()
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
                                class("prev_page"),
                                r#type("button"),
                                disabled(self.page <= 1),
                                value("<< Prev Page"),
                                on_click(|_| {
                                    trace!("Button is clicked");
                                    Msg::PrevPage
                                }),
                            ],
                            vec![],
                        ),
                        text(format!("Page: {}", self.page)),
                        input(
                            vec![
                                class("next_page"),
                                r#type("button"),
                                disabled(self.page >= 2),
                                value("Next Page >>"),
                                on_click(|_| {
                                    trace!("Button is clicked");
                                    Msg::NextPage
                                }),
                            ],
                            vec![],
                        ),
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
                footer(
                    vec![class("error")],
                    vec![if let Some(error) = &self.error {
                        text(error)
                    } else {
                        text!("")
                    }],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::NextPage => {
                if self.page < 2 {
                    self.page += 1;
                }
                self.fetch_page()
            }
            Msg::PrevPage => {
                if self.page > 1 {
                    self.page -= 1;
                }
                self.fetch_page()
            }
            Msg::ReceivedData(data) => {
                self.data = data;
                Cmd::none()
            }
            Msg::JsonError(err) => {
                trace!("Error fetching users! {:#?}", err);
                self.error = Some(format!(
                    "There was an error fetching the page: {:?}",
                    err
                ));
                Cmd::none()
            }
            Msg::RequestError(type_error) => {
                trace!("Error requesting the page: {:?}", type_error);
                self.error = Some(format!(
                    "There was an error fetching the page: {:?}",
                    type_error
                ));
                Cmd::none()
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
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
