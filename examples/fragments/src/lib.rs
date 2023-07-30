use sauron::dom::{delay, spawn_local};
use sauron::{html::fragment, *};

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}

#[derive(Default)]
struct App {
    items: Vec<Node<Msg>>,
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                delay(1000).await;
                program.dispatch(Msg::AddItem);
                delay(2000).await;
                program.dispatch(Msg::AddItem);
                delay(3000).await;
                program.dispatch(Msg::AddItem);
                delay(4000).await;
                program.dispatch(Msg::AddItem);
                delay(5000).await;
                program.dispatch(Msg::AddItem);
            })
        })
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
        match msg {
            Msg::AddItem => self
                .items
                .push(node! { <li>{text(self.items.len() + 1)}</li> }),
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

enum Msg {
    AddItem,
}
