use crate::prelude::*;

#[wasm_bindgen(module = "/js/define_custom_element.js")]
extern "C" {
    /// register using custom element define
    /// # Example:
    /// ```rust,ignore
    ///  sauron::register_custom_element("date-time", "DateTimeWidgetCustomElement", "HTMLElement");
    /// ```
    pub fn register_custom_element(
        custom_tag: &str,
        adapter: &str,
        superClass: &str,
    );
}
