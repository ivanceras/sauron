use crate::prelude::*;

#[wasm_bindgen(module = "/js/define_custom_element.js")]
extern "C" {
    // register using custom element define
    // # Example:
    // ```rust,ignore
    //  sauron::register_custom_element("date-time", "DateTimeWidgetCustomElement", "HTMLElement");
    // ```
    pub fn register_custom_element(
        custom_tag: &str,
        adapter: &str,
        superClass: &str,
    );
}

/// a shim to stringify the adapter from the ident generated using concat_idents
#[macro_export]
macro_rules! stringify_generated_component_adaptor {
    ($krate:ident, $module:ident,$widget:ident) => {
        concat!(
            stringify!($krate),
            "_",
            stringify!($module),
            "_",
            stringify!($widget),
            "_CustomElement"
        )
    };
}

/// declare a custom element
/// TODO: CustomMsg will conflict with other component's in the same crate's CustomMsg
/// maybe use syn to parse the InputStream
#[macro_export]
macro_rules! declare_custom_element {
    ($custom_tag:tt, $krate:ident, $module:ident, $widget:ident, $msg:ident) => {
        concat_idents::concat_idents!(component_adaptor = $krate, _, $module, _, $widget, _, CustomElement {

            #[derive(Debug, Clone)]
            struct CustomMsg(crate::$module::$msg);

            #[allow(non_camel_case_types)]
            #[wasm_bindgen]
            pub struct component_adaptor{
                program: Program<$widget<CustomMsg>, CustomMsg>,
            }

            #[wasm_bindgen]
            #[allow(non_camel_case_types)]
            impl component_adaptor {
                #[wasm_bindgen(constructor)]
                pub fn new(node: JsValue) -> Self {
                    log::info!("constructor..");
                    let mount_node: &web_sys::Node = node.unchecked_ref();
                    Self {
                        program: Program::new(
                            $widget::default(),
                            mount_node,
                            false,
                            true,
                        ),
                    }
                }

                #[wasm_bindgen(method)]
                pub fn observed_attributes() -> JsValue {
                    JsValue::from_serde(&$widget::<CustomMsg>::observed_attributes())
                        .expect("must parse from serde")
                }

                #[wasm_bindgen(method)]
                pub fn attribute_changed_callback(&self) {
                    log::info!("attribute changed...");
                    let mount_node = self.program.mount_node();
                    let mount_element: &web_sys::Element = mount_node.unchecked_ref();
                    let attribute_names = mount_element.get_attribute_names();
                    let len = attribute_names.length();
                    let mut attribute_values: BTreeMap<String, String> = BTreeMap::new();
                    for i in 0..len {
                        let name = attribute_names.get(i);
                        let attr_name =
                            name.as_string().expect("must be a string attribute");
                        if let Some(attr_value) = mount_element.get_attribute(&attr_name) {
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
                    self.program.mount();
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

            impl Application<CustomMsg> for $widget<CustomMsg> {
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

            #[wasm_bindgen]
            pub fn register_components() {
                sauron::register_custom_element(
                    $custom_tag,
                    stringify_generated_component_adaptor!($krate, $module, $widget),
                    "HTMLElement"
                );
            }

        });

    };
}
