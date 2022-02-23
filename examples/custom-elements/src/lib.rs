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
        div(
            [],
            [
                p([], [self.view_date(2012, 5)]),
                select(
                    [on_change(|input| Msg::LanguageChanged(input.value))],
                    [
                        option([value("sr-RS")], [text("sr-RS")]),
                        option([value("en-GB")], [text("en-GB")]),
                        option([value("en-US")], [text("en-US")]),
                    ],
                ),
            ],
        )
    }
}

impl App {
    fn view_date(&self, year: i32, month: i32) -> Node<Msg> {
        html_element(
            None,
            "intl-date",
            [
                attr("lang", &self.language),
                attr("year", year),
                attr("month", month),
            ],
            [],
            false,
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
