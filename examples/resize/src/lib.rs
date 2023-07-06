#![deny(warnings)]
#![deny(clippy::all)]
use sauron::{html::*, *};

pub enum Msg {
    Click,
    NoOp,
}

#[derive(Default)]
pub struct App {
    click_count: u32,
}

impl Application<Msg> for App {
    fn init(&mut self) -> Vec<Cmd<Self, Msg>> {
        vec![sauron::dom::Window::on_resize_task(|w, h| {
            log::info!("Window is resized to {w}x{h}");
            Msg::NoOp
        })
        .into()]
    }

    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h1([], [text("Usage of task")]),
                div([], [text("resize the window")]),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::NoOp => (),
        }
        Cmd::none()
    }

    fn style(&self) -> String {
        jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
