#![deny(warnings)]

pub use date_time::DateTimeWidget;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use std::collections::BTreeMap;

mod date_time;

#[derive(Debug, Clone)]
pub struct CustomMsg(date_time::Msg);

#[wasm_bindgen]
pub struct DateTimeWidgetCustomElement {
    program: Program<DateTimeWidget<CustomMsg>, CustomMsg>,
}

#[wasm_bindgen]
impl DateTimeWidgetCustomElement {
    #[wasm_bindgen(constructor)]
    pub fn new(node: JsValue) -> Self {
        log::info!("constructor..");
        let root_node: &web_sys::Node = node.unchecked_ref();
        Self {
            program: Program::new(DateTimeWidget::default(), root_node),
        }
    }

    #[wasm_bindgen(method)]
    pub fn observed_attributes() -> JsValue {
        JsValue::from_serde(&DateTimeWidget::<CustomMsg>::observed_attributes())
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
            .attributes_changed(attribute_values);
    }

    #[wasm_bindgen(method)]
    pub fn connected_callback(&mut self) {
        self.program.start_append_to_mount();
        log::info!("Component is connected..");
        self.program.update_dom();
    }
    #[wasm_bindgen(method)]
    pub fn disconnected_callback(&mut self) {
        log::info!("Component is disconnected..");
    }
    #[wasm_bindgen(method)]
    pub fn adopted_callback(&mut self) {
        log::info!("Component is adopted..");
    }
}

impl Application<CustomMsg> for DateTimeWidget<CustomMsg> {
    fn update(&mut self, msg: CustomMsg) -> Cmd<Self, CustomMsg> {
        let mount_attributes = self.attributes_for_mount();
        Cmd::batch([
            Cmd::from(
                <Self as Component<date_time::Msg, CustomMsg>>::update(
                    self, msg.0,
                )
                .localize(CustomMsg),
            ),
            Cmd::new(|program| {
                log::info!("mount attributes: {:?}", mount_attributes);
                program.update_mount_attributes(mount_attributes);
            }),
        ])
    }

    fn view(&self) -> Node<CustomMsg> {
        <Self as Component<date_time::Msg, CustomMsg>>::view(self)
            .map_msg(CustomMsg)
    }
}

/// registers the custom element
/// This must be called upon loading the wasm
#[wasm_bindgen]
pub fn register_components() {
    sauron::register_custom_element(
        "date-time",
        "DateTimeWidgetCustomElement",
        "HTMLElement",
    );
}
