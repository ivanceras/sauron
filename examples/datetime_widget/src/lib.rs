#![deny(warnings)]

pub use date_time::DateTimeWidget;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use std::collections::BTreeMap;

mod date_time;

#[derive(Debug, Clone)]
pub struct Msg(date_time::Msg);

#[wasm_bindgen]
pub struct DateTimeWidgetCustomElement {
    is_mounted: bool,
    program: Program<DateTimeWidget<Msg>, Msg>,
}

#[wasm_bindgen]
impl DateTimeWidgetCustomElement {
    #[wasm_bindgen(constructor)]
    pub fn new(node: JsValue) -> Self {
        log::info!("constructor..");
        let root_node: &web_sys::Node = node.unchecked_ref();
        Self {
            is_mounted: false,
            program: Program::append_to_mount(
                DateTimeWidget::default(),
                root_node,
            ),
        }
    }

    #[wasm_bindgen(method)]
    pub fn observed_attributes() -> JsValue {
        JsValue::from_serde(&DateTimeWidget::<Msg>::observed_attributes())
            .expect("must parse from serde")
    }

    #[wasm_bindgen(method)]
    pub fn attribute_changed_callback(&self) {
        log::info!("attribute changed...");
        let mount_node = self.program.mount_node();
        let mount_elm: &web_sys::Element = mount_node.unchecked_ref();
        let attribute_names = mount_elm.get_attribute_names();
        let len = attribute_names.length();
        let mut attribute_values: BTreeMap<String, String> = BTreeMap::new();
        for i in 0..len {
            let name = attribute_names.get(i);
            let attr_name =
                name.as_string().expect("must be a string attribute");
            if let Some(attr_value) = mount_elm.get_attribute(&attr_name) {
                attribute_values.insert(attr_name, attr_value);
            }
        }
        self.program
            .app
            .borrow_mut()
            .attribute_changed(attribute_values);
        self.program.update_dom();
    }

    #[wasm_bindgen(method)]
    pub fn connected_callback(&mut self) {
        self.is_mounted = true;
        log::info!("Component is connected..");
        //self.attribute_changed_callback();
        //self.program.start_append_to_mount();
    }
    #[wasm_bindgen(method)]
    pub fn disconnected_callback(&mut self) {
        self.is_mounted = false;
        log::info!("Component is disconnected..");
    }
    #[wasm_bindgen(method)]
    pub fn adopted_callback(&mut self) {
        self.is_mounted = true;
        log::info!("Component is adopted..");
    }
}

impl Application<Msg> for DateTimeWidget<Msg> {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        Cmd::from(
            <Self as Component<date_time::Msg, Msg>>::update(self, msg.0)
                .localize(Msg),
        )
    }

    fn view(&self) -> Node<Msg> {
        <Self as Component<date_time::Msg, Msg>>::view(self).map_msg(Msg)
    }
}

#[wasm_bindgen(module = "/define_custom_element.js")]
extern "C" {
    pub fn register_custom_element(custom_tag: &str, adapter: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    register_custom_element("date-time", "DateTimeWidgetCustomElement");
    log::info!("loaded...");
}
