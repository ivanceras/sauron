#![deny(warnings)]
use datetime_widget::date_time::{date, date_time, time};
use sauron::*;

enum AppMsg {}

#[derive(Default)]
pub struct App {}

impl Application<AppMsg> for App {
    fn update(&mut self, _msg: AppMsg) -> Cmd<Self, AppMsg> {
        Cmd::none()
    }

    fn view(&self) -> Node<AppMsg> {
        node! {
            <div>
                <h2>"Called using `date-time` tag "</h2>
                <date-time date="2022-05-16" time="15:46"></date-time>
                <h3>"Using in an expression date_time([],[])"</h3>
                {date_time([date("2022-07-07"), time("07:07")],[])}
            </div>
        }
    }

    fn style(&self) -> String {
        "".to_string()
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
