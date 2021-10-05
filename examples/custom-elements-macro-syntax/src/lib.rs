/// Adapted from: https://github.com/elm-community/js-integration-examples/blob/master/internationalization/src/Main.elm
use sauron::prelude::*;

enum Msg {
    LanguageChanged(String),
}

struct App {
    language: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            language: "sr-Sr".to_string(),
        }
    }
}

impl Application<Msg> for App {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::LanguageChanged(language) => {
                self.language = language;
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <div>
                <p>{self.view_date(2012, 5)}</p>
                <select on_change={|input| Msg::LanguageChanged(input.value)}>
                    <option value="sr-RS">"sr-RS"</option>
                    <option value="en-GB">"en-GB"</option>
                    <option value="en-US">"en-US"</option>
                </select>
            </div>
        }
    }
}

impl App {
    fn view_date(&self, year: i32, month: i32) -> Node<Msg> {
        node! {
            <intl-date lang={&self.language} year={year} month={month} />
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
