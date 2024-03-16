#![deny(warnings)]
#![deny(clippy::all)]
use sauron::{html::*, *};

pub enum Msg {
    WindowResized(i32, i32),
}

#[derive(Default)]
pub struct App {
    width: Option<i32>,
    height: Option<i32>,
}

impl Application for App {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self> {
        Cmd::from(Window::on_resize(|w, h| {
            log::info!("This will trigger only once.. {w}x{h}");
            Msg::WindowResized(w, h)
        }))
    }

    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h1([], [text("Usage of task")]),
                ol(
                    [],
                    [
                        li([], [text("resize the window")]),
                        li([], [text("open the console")]),
                        if let (Some(w), Some(h)) = (self.width, self.height) {
                            li(
                                [],
                                [text!("See the log that the window is resized to {w} x {h}")],
                            )
                        } else {
                            span([], [])
                        },
                    ],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::WindowResized(w, h) => {
                log::info!("Setting the App's width: {w} and height: {h}");
                self.width = Some(w);
                self.height = Some(h);
                Cmd::none()
            }
        }
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
    Program::mount_to_body(App::default());
}
