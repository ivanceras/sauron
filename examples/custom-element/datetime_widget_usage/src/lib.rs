#![deny(warnings)]
use sauron::prelude::*;

enum AppMsg {}

#[derive(Default)]
pub struct App {}

#[custom_element("lee-app")]
impl Application<AppMsg> for App {
    fn update(&mut self, _msg: AppMsg) -> Cmd<Self, AppMsg> {
        Cmd::none()
    }

    fn view(&self) -> Node<AppMsg> {
        node! {
            <div>
                <h5>"Usage of custom element"</h5>
                <date-time date="2022-05-16" time="15:46"></date-time>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    log::info!("loaded...");
    datetime_widget::date_time::register();
    Program::mount_to_body(App::default());
}
