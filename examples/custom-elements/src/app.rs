use crate::web_sys;
use crate::web_sys::Element;
use crate::web_sys::HtmlElement;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use std::collections::BTreeMap;
#[derive(Debug)]
enum Msg {
    LanguageChanged(String),
}

#[wasm_bindgen]
pub struct AppCE {
    program: Program<App, Msg>,
}

#[wasm_bindgen]
impl AppCE {
    #[wasm_bindgen(constructor)]
    pub fn new(node: JsValue) -> Self {
        let root_node: &web_sys::Node = node.unchecked_ref();
        log::info!("The app is mounted here..");
        log::info!("root_node: {:#?}", root_node);
        let program = Program::new(App::default(), root_node);
        Self { program }
    }

    #[wasm_bindgen(method)]
    pub fn observed_attributes() -> JsValue {
        JsValue::from_serde(&App::observed_attributes())
            .expect("must parse from serde")
    }

    #[wasm_bindgen(method)]
    pub fn attribute_changed_callback(&self) {
        let root_node = self.program.root_node();
        let root_elm: &Element = root_node.unchecked_ref();
        let attribute_names = root_elm.get_attribute_names();
        let len = attribute_names.length();
        let mut attribute_values: BTreeMap<String, String> = BTreeMap::new();
        for i in 0..len {
            let name = attribute_names.get(i);
            let attr_name =
                name.as_string().expect("must be a string attribute");
            if let Some(attr_value) = root_elm.get_attribute(&attr_name) {
                attribute_values.insert(attr_name, attr_value);
            }
        }
        log::info!("before app: {:#?}", self.program.app.borrow());
        self.program
            .app
            .borrow_mut()
            .attribute_changed(attribute_values);
        log::info!("after app: {:#?}", self.program.app.borrow());
    }

    #[wasm_bindgen(method)]
    pub fn connected_callback(&self) {
        log::info!("Component is connected..");
        self.program.start_append_to_mount();
    }
    #[wasm_bindgen(method)]
    pub fn disconnected_callback(&self) {
        log::info!("Component is disconnected..");
    }
    #[wasm_bindgen(method)]
    pub fn adopted_callback(&self) {
        log::info!("Component is adopted..");
    }
}

#[derive(Debug)]
pub struct App {
    language: String,
    year: String,
    month: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            language: "sr-Sr".to_string(),
            year: "".to_string(),
            month: "".to_string(),
        }
    }
}

impl Application<Msg> for App {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        log::info!("app is updated: {:#?}", msg);
        match msg {
            Msg::LanguageChanged(language) => {
                self.language = language;
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        p(
            [],
            [
                //p([], [self.view_date(2012, 5)]),
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
            "local-date",
            [
                attr("lang", &self.language),
                attr("year", year),
                attr("month", month),
            ],
            [],
            false,
        )
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["lang", "year", "month"]
    }

    fn attribute_changed(&mut self, attributes: BTreeMap<String, String>) {
        log::info!(
            "custome element has changed its attributes: {:#?}",
            attributes
        );
        for (key, value) in attributes {
            match &*key {
                "lang" => self.language = value,
                "year" => self.year = value,
                "month" => self.month = value,
                _ => log::info!("unused attribute: {}", key),
            }
        }
    }
}
