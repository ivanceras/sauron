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
    is_mounted: bool,
    program: Program<DateTimeWidget<CustomMsg>, CustomMsg>,
}

#[wasm_bindgen]
impl DateTimeWidgetCustomElement {
    #[wasm_bindgen(constructor)]
    pub fn new(node: JsValue) -> Self {
        log::info!("constructor..");
        let root_node: &web_sys::Node = node.unchecked_ref();
        Self {
            is_mounted: false,
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
        self.is_mounted = true;
        log::info!("Component is connected..");
        self.program.update_dom();
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

#[wasm_bindgen(module = "/define_custom_element.js")]
extern "C" {
    /// register using custom element define
    /// # Example:
    /// ```rust
    ///  register_custom_element("date-time", "DateTimeWidgetCustomElement", "HTMLElement");
    /// ```
    pub fn register_custom_element(
        custom_tag: &str,
        adapter: &str,
        superClass: &str,
    );
}

/// registers the custom element
/// This must be called upon loading the wasm
#[wasm_bindgen]
pub fn register_components() {
    register_custom_element(
        "date-time",
        "DateTimeWidgetCustomElement",
        "HTMLElement",
    );
}

pub enum AppMsg {
    Nothing,
}

#[derive(Default)]
pub struct App {}

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
    register_components();
    Program::mount_to_body(App::default());
}
