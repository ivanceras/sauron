use sauron::*;

struct App;

impl Application<()> for App {
    fn view(&self) -> Node<()> {
        let count = 0;
        node! {
            <p id="p1" on_click=|_|{log::info!("hello")} value=count>
                Hello World!
                <!-- "This is a comment" -->
            </p>
        }
    }

    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    Program::mount_to_body(App);
}
