use crate::wasm_bindgen;
#[cfg(feature = "with-dom")]
use wasm_bindgen::JsValue;

#[cfg(feature = "with-dom")]
use crate::dom::Dispatch;

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
    #[cfg(feature = "with-dom")]
    fn attribute_changed<DSP>(
        program: &DSP,
        attr_name: &str,
        old_value: JsValue,
        new_value: JsValue,
    ) where
        DSP: Dispatch<MSG> + Clone + 'static;
}
