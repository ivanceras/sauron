use sauron::dom::{async_delay, spawn_local};
use sauron::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}

#[derive(Default)]
struct App {
    show: bool,
}

impl Application<Msg> for App {
    fn init(&mut self) -> Vec<Cmd<Self, Msg>> {
        vec![Cmd::new(|program| {
            spawn_local(async move {
                program.dispatch(Msg::ToggleShow);
            })
        })]
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ToggleShow => {
                self.show = !self.show;
                if self.show {
                    document().set_title("Now, you see me...");
                } else {
                    document().set_title("Now, you don't!");
                }
                Cmd::new(|program| {
                    spawn_local(async move {
                        async_delay(2000).await.expect("error");
                        program.dispatch(Msg::ToggleShow);
                    })
                })
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

enum Msg {
    ToggleShow,
}
