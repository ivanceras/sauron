use frame::Frame;
use sauron::*;
use status::Status;
use theme::Theme;

mod frame;
mod status;
mod theme;

#[derive(Default)]
enum Msg {
    Clicked,
    #[default]
    NoOp,
}

struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
            {stateful_component(Frame::default(), [], [
                button([on_click(|_|Msg::Clicked)],[text!("Button has been clicked {} times", self.count)])
            ])}
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Clicked => {
                log::info!("Button has been clicked...");
                self.count += 1;
            }
            Msg::NoOp => (),
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        let mut main = vec![jss! {
            "body":{
                font_family: "verdana, arial, monospace",
            },

            "main":{
                width:px(30),
                height: px(100),
                margin: "auto",
                text_align: "center",
            },

            "input, .count":{
                font_size: px(40),
                padding: px(30),
                margin: px(30),
            }
        }];

        main
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}

