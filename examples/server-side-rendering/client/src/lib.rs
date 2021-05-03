use sauron::prelude::*;
use sauron::js_sys::TypeError;
use sauron::web_sys::Response;
use serde::{
    Deserialize,
    Serialize,
};

#[macro_use]
extern crate log;

const DATA_URL: &'static str = "http://localhost:3030/api";

#[derive(Debug, Serialize, Deserialize)]
pub enum FetchStatus<T> {
    Idle(T),
    Loading,
    Complete(T),
    Error(Option<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    EditName(String),
    ReceivedData(Result<Data, Response>),
    RequestError(TypeError),
    QueryAPI,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Data {
    pub length: usize,
    pub modified_name: String,
}

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub data: FetchStatus<Data>,
    error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            name: String::from(""),
            data: FetchStatus::Idle(Data {
                length: 0,
                modified_name: String::from(""),
            }),
            error: None,
        }
    }

    fn fetch_data(&self) -> Cmd<Self, Msg> {
        let url = format!("{}/{}", DATA_URL, self.name);
        Http::fetch_with_text_response_decoder(
            &url,
            |v: String| {
                let data: Result<Data, _> = serde_json::from_str(&v);
                trace!("data: {:#?}", data);
                data.expect("Error deserializing data")
            },
            Msg::ReceivedData,
            Msg::RequestError,
        )
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <form on_submit={|e| {
                    e.prevent_default();
                    Msg::QueryAPI
                }}>
                    <label>
                        "What’s your name, man?"
                        <input
                            type="text"
                            value={&self.name}
                            on_input={|e| Msg::EditName(e.value)}
                        />
                    </label>
                    <button type="submit">"Okay!"</button>
                </form>
                {self.view_data()}
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("App is updating from msg: {:?}", msg);
        let mut cmd = Cmd::none();
        match msg {
            Msg::EditName(name) => self.name = name,
            Msg::QueryAPI => {
                self.data = FetchStatus::Loading;
                cmd = self.fetch_data()
            }
            Msg::ReceivedData(Ok(data)) => {
                self.data = FetchStatus::Complete(data)
            }
            Msg::ReceivedData(Err(js_value)) => {
                trace!("Error fetching data! {:#?}", js_value);
                self.data = FetchStatus::Error(Some(format!(
                    "There was an error reaching the api: {:?}",
                    js_value
                )));
            }
            Msg::RequestError(type_error) => {
                trace!("Error requesting the page: {:?}", type_error);
                self.data = FetchStatus::Error(Some(format!(
                    "There was an error fetching the page: {:?}",
                    type_error
                )));
            }
        };
        cmd
    }
}

impl App {
    fn view_data(&self) -> Node<Msg> {
        match &self.data {
            FetchStatus::Idle(_) => node! { <p>"Waiting around..."</p> },
            FetchStatus::Error(Some(e)) => {
                node! {
                    <article>
                        <p>"Okay, something went wrong. I think it was: "</p>
                        <code>{text(e)}</code>
                    </article>
                }
            }
            FetchStatus::Error(None) => {
                node! {
                    <article>
                        <p>"Okay, something went wrong. I have no idea what it is."</p>
                    </article>
                }
            }
            FetchStatus::Loading => {
                node! {
                    <article>
                        <p>"Loading..."</p>
                    </article>
                }
            }
            FetchStatus::Complete(data) => {
                node! {
                    <article>
                        <p>"Hello, "<span class="modified-name">{text(&data.modified_name)}</span></p>
                        <p>"Hey, did you know that’s "<span class="length">{text(&data.length)}</span>" characters long?"</p>
                    </article>
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    /* Deserialize starting app data from the argument, which is passed in index.html
     * (but generated in server/src/main.rs) */
    let mut app = App::new();
    if let Ok(state) = serde_json::from_str::<App>(&serialized_state) {
        app.name = state.name;
        app.data = state.data;
    };

    /* If there's a window (i.e., if this is running in the browser)
     * then mount the app by swapping out the <main> tag */
    match web_sys::window() {
        Some(window) => {
            trace!("found window, will try to replace <main>");
            let document =
                window.document().expect("should have a document on window");
            Program::new_replace_mount(
                app,
                &document.query_selector_all("main").unwrap().get(0).unwrap(),
            );
        }
        None => {
            trace!("window not found");
            Program::mount_to_body(app);
        }
    }
}
