use sauron::dom::{delay, spawn_local};
use sauron::{html::fragment, *};

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}

enum Msg {
    AddItem,
}

#[derive(Default)]
struct App {
    items: Vec<Node<Msg>>,
}

impl Application for App {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self> {
        Cmd::new(|mut program| {
            spawn_local(async move {
                loop {
                    delay(1000).await;
                    program.dispatch(Msg::AddItem);
                }
            })
        })
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self>
    where
        Self: Sized + 'static,
    {
        match msg {
            Msg::AddItem => self
                .items
                .push(node! { <li>{text!("Item {}", self.items.len() + 1)}</li> }),
        }

        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
          <div>
            {fragment(self.items.iter().cloned().chain([node! {<span />}]))}
          </div>
        }
    }
}

