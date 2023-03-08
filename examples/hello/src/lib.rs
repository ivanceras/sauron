use sauron::prelude::*;

struct App;

#[async_trait(?Send)]
impl Application<()> for App {
    fn view(&self) -> Node<()> {
        node! {
            <p>
                "hello"
            </p>
        }
    }

    async fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        Cmd::none()
    }

    fn style(&self) -> String {
        String::new()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App);
}
