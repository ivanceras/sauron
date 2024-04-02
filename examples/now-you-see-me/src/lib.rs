use sauron::dom::{delay, spawn_local};
use sauron::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}

enum Msg {
    ToggleShow,
}

#[derive(Default)]
struct App {
    show: bool,
}

impl Application for App {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Msg> {
        Cmd::single( async move{
            Msg::ToggleShow
        })
    }
    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::ToggleShow => {
                self.show = !self.show;
                if self.show {
                    document().set_title("Now, you see me...");
                } else {
                    document().set_title("Now, you don't!");
                }
                Cmd::single(
                    async move {
                        delay(2000).await;
                        Msg::ToggleShow
                    }
                )
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        if self.show {
            node! { <h1>Now you see me...</h1> }
        } else {
            node! { <> </> }
        }
    }
}
