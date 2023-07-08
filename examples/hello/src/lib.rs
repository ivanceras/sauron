use sauron::*;

struct App;

impl Application<()> for App {
    fn init(&mut self) -> Vec<Cmd<Self, ()>> {
        vec![]
    }
    fn view(&self) -> Node<()> {
        node! {
            <p>
                "hello"
            </p>
        }
    }

    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        Cmd::none()
    }

    fn style(&self) -> Vec<String> {
        vec![]
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App);
}
