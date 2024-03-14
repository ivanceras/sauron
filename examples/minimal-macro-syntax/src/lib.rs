#![deny(warnings)]
use sauron::{html::attributes::*, html::*, jss, *};

#[derive(Debug)]
enum Msg {
    Click,
}

struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Application for App {

    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>Minimal example</h1>
                <div class="some-class" id="some-id" {attr("data-id", 1)}>
                    <input class="client"
                            type="button"
                            value="Click me!"
                            key=1
                            on_click=|_| {
                                log::trace!("Button is clicked");
                                Msg::Click
                            }
                    />
                    <div>{text(format!("Clicked: {}", self.click_count))}</div>
                    <input type="text" value=self.click_count/>
                </div>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        log::trace!("App is updating with msg: {:?}", msg);
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
