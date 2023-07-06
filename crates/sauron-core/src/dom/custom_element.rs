use crate::dom::{MountAction, MountTarget};
use crate::wasm_bindgen;
use crate::{Application, Program};
use wasm_bindgen::JsValue;

#[wasm_bindgen(module = "/js/define_custom_element.js")]
extern "C" {
    // register using custom element define
    // # Example:
    // ```rust,ignore
    //  sauron::register_custom_element("date-time", "DateTimeWidgetCustomElement");
    // ```
    pub fn register_custom_element(custom_tag: &str, adapter: &str);
}

/// a trait for implementing CustomElement in the DOM with custom tag
pub trait CustomElement<MSG> {
    /// the custom tag that this custom element will be registerd to the browser
    fn custom_tag() -> &'static str;

    /// returns the attributes that is observed by this component
    /// These are the names of the attributes the component is interested in
    fn observed_attributes() -> Vec<&'static str>;

    /// This will be invoked when a component is used as a custom element
    /// and the attributes of the custom-element has been modified
    ///
    /// if the listed attributes in the observed attributes are modified
    fn attribute_changed<APP>(
        program: &Program<APP, MSG>,
        attr_name: &str,
        old_value: JsValue,
        new_value: JsValue,
    ) where
        APP: Application<MSG> + 'static,
        MSG: 'static;

    /// the component is attached to the dom
    fn connected_callback(&mut self);
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self);

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self);
}

/// A self contain web component
/// This is needed to move some of the code from the #web_component macro
/// This is also necessary, since #[wasm_bindgen] macro can not process impl types which uses
/// generics, we use generics here to simplify the code and do the type checks for us, rather than
/// in the code derived from the #[web_component] macro
pub struct WebComponent<APP, MSG>
where
    MSG: 'static,
{
    program: Program<APP, MSG>,
}

impl<APP, MSG> WebComponent<APP, MSG>
where
    APP: Application<MSG> + Default + 'static,
    APP: CustomElement<MSG>,
    MSG: 'static,
{
    /// create a new web component, with the node as the target element to be mounted into
    pub fn new(node: JsValue) -> Self {
        use crate::wasm_bindgen::JsCast;
        let mount_node: &web_sys::Node = node.unchecked_ref();
        Self {
            program: Program::new(
                APP::default(),
                mount_node,
                MountAction::Append,
                MountTarget::ShadowRoot,
            ),
        }
    }

    /// When the attribute of the component is changed, this method will be called
    pub fn attribute_changed(&self, attr_name: &str, old_value: JsValue, new_value: JsValue) {
        APP::attribute_changed(&self.program, attr_name, old_value, new_value)
    }

    /// called when the web component is mounted
    pub fn connected_callback(&mut self) {
        self.program.mount();
        let component_style = <APP as Application<MSG>>::style(&self.program.app.borrow()).join("");
        self.program.inject_style_to_mount(&component_style);
        self.program.update_dom().expect("must update dom");
        self.program.app.borrow_mut().connected_callback();
    }

    /// called when the web component is removed
    pub fn disconnected_callback(&mut self) {
        self.program.app.borrow_mut().disconnected_callback();
    }

    /// called when web componented is moved into other parts of the document
    pub fn adopted_callback(&mut self) {
        self.program.app.borrow_mut().adopted_callback();
    }
}
