use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node, Program};

#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    count: i32,
}

impl App {
    pub fn new() -> Self {
        App { count: 0 }
    }
}

impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    key="inc"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <div class="count">{text(self.count)}</div>
                <input type="button"
                    value="-"
                    key="dec"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App::new());
}
