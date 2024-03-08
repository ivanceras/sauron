use crate::vdom::Attribute;
use crate::vdom::AttributeName;
use crate::vdom::AttributeValue;
use crate::vdom::Namespace;
use crate::vdom::Style;
use crate::vdom::Value;
use crate::vdom::{CHECKED, DISABLED, OPEN, VALUE};
use wasm_bindgen::intern;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::Element;
use web_sys::{
    self, HtmlButtonElement, HtmlDataElement, HtmlDetailsElement, HtmlFieldSetElement,
    HtmlInputElement, HtmlLiElement, HtmlLinkElement, HtmlMeterElement, HtmlOptGroupElement,
    HtmlOptionElement, HtmlOutputElement, HtmlParamElement, HtmlProgressElement, HtmlSelectElement,
    HtmlStyleElement, HtmlTextAreaElement, 
};

/// a dom version of the Attribute, thereby removing the MSG generic
pub struct DomAttr {
    /// namespace of the attribute
    pub namespace: Option<&'static str>,
    /// the name of the attribute
    pub name: &'static str,
    /// the value of the attribute
    pub value: Vec<DomAttrValue>,
}

/// a dom version of the Attribute value, thereby removing the MSG generic
pub enum DomAttrValue {
    /// function calls
    FunctionCall(Value),
    /// simple value
    Simple(Value),
    /// a style
    Style(Vec<Style>),
    /// event listeners
    EventListener(Closure<dyn FnMut(web_sys::Event)>),
    /// an empty value, can also represents null values from JsValue
    Empty,
}

/// a struct where the listeners, plain values, styles and function call values are grouped
/// separately
pub struct GroupedDomAttrValues {
    /// the listeners of the event listeners
    pub listeners: Vec<Closure<dyn FnMut(web_sys::Event)>>,
    /// plain attribute values
    pub plain_values: Vec<Value>,
    /// style attribute values
    pub styles: Vec<Style>,
    /// function calls
    pub function_calls: Vec<Value>,
}

impl DomAttr {

    /// return the values grouped into listeners, plain, styles and function calls
    pub(crate) fn group_values(self) -> GroupedDomAttrValues {
        let mut listeners = vec![];
        let mut plain_values = vec![];
        let mut styles = vec![];
        let mut function_calls = vec![];
        for av in self.value {
            match av {
                DomAttrValue::Simple(v) => {
                    plain_values.push(v);
                }
                DomAttrValue::FunctionCall(v) => {
                    function_calls.push(v);
                }
                DomAttrValue::Style(s) => {
                    styles.extend(s);
                }
                DomAttrValue::EventListener(cb) => {
                    listeners.push(cb);
                }
                DomAttrValue::Empty => (),
            }
        }
        GroupedDomAttrValues {
            listeners,
            plain_values,
            styles,
            function_calls,
        }
    }

    pub(crate) fn convert_attr_except_listener<MSG>(attr: &Attribute<MSG>) -> DomAttr {
        DomAttr {
            namespace: attr.namespace,
            name: attr.name,
            value: attr
                .value
                .iter()
                .filter_map(|v| DomAttrValue::convert_attr_value_except_listener(v))
                .collect(),
        }
    }

    /// Note: Used only templates
    /// set the lement with dom attr except for the event listeners
    pub(crate) fn set_element_dom_attr_except_listeners(element: &Element, attr: DomAttr) {
        let attr_name = intern(attr.name);
        let attr_namespace = attr.namespace;

        let GroupedDomAttrValues {
            listeners: _,
            plain_values,
            styles,
            function_calls,
        } = attr.group_values();
        Self::set_element_style(element, attr_name, styles);
        Self::set_element_function_call_values(element, attr_name, function_calls);
        Self::set_element_simple_values(element, attr_name, attr_namespace, plain_values);
    }

    /// set the style of this element
    pub(crate) fn set_element_style(element: &Element, attr_name: AttributeName, styles: Vec<Style>) {
        if let Some(merged_styles) = Style::merge_to_string(&styles) {
            // set the styles
            element
                .set_attribute(attr_name, &merged_styles)
                .unwrap_or_else(|_| panic!("Error setting an attribute_ns for {element:?}"));
        } else {
            //if the merged attribute is blank of empty when string is trimmed
            //remove the attribute
            element
                .remove_attribute(attr_name)
                .expect("must remove attribute");
        }
    }

    /// do function calls such as set_inner_html
    pub(crate) fn set_element_function_call_values(
        element: &Element,
        attr_name: AttributeName,
        function_calls: Vec<Value>,
    ) {
        if let Some(merged_func_values) = Value::merge_to_string(function_calls.iter()) {
            if attr_name == "inner_html" {
                element.set_inner_html(&merged_func_values);
            }
        }
    }

    /// set simple values
    pub(crate) fn set_element_simple_values(
        element: &Element,
        attr_name: AttributeName,
        attr_namespace: Option<Namespace>,
        plain_values: Vec<Value>,
    ) {
        if let Some(merged_plain_values) = Value::merge_to_string(plain_values.iter()) {
            if let Some(namespace) = attr_namespace {
                // Warning NOTE: set_attribute_ns should only be called
                // when you meant to use a namespace
                // using this with None will error in the browser with:
                // NamespaceError: An attempt was made to create or change an object in a way which is incorrect with regard to namespaces
                element
                    .set_attribute_ns(Some(namespace), attr_name, &merged_plain_values)
                    .unwrap_or_else(|_| panic!("Error setting an attribute_ns for {element:?}"));
            } else {
                if *VALUE == attr_name {
                    element
                        .set_attribute(attr_name, &merged_plain_values)
                        .unwrap_or_else(|_| panic!("Error setting an attribute for {element:?}"));
                    Self::set_value_str(element, &merged_plain_values);
                    Self::set_numeric_values(element, &plain_values);
                } else if *OPEN == attr_name {
                    let is_open: bool = plain_values
                        .first()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    element
                        .set_attribute(attr_name, &is_open.to_string())
                        .unwrap_or_else(|_| panic!("Error setting an attribute for {element:?}"));
                    Self::set_open(element, is_open);
                } else if *CHECKED == attr_name {
                    let is_checked: bool = plain_values
                        .first()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    element
                        .set_attribute(attr_name, &is_checked.to_string())
                        .unwrap_or_else(|_| panic!("Error setting an attribute for {element:?}"));
                    Self::set_checked(element, is_checked)
                } else if *DISABLED == attr_name {
                    let is_disabled: bool = plain_values
                        .first()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    element
                        .set_attribute(attr_name, &is_disabled.to_string())
                        .unwrap_or_else(|_| panic!("Error setting an attribute for {element:?}"));
                    Self::set_disabled(element, is_disabled);
                } else {
                    element
                        .set_attribute(attr_name, &merged_plain_values)
                        .unwrap_or_else(|_| panic!("Error setting an attribute for {element:?}"));
                }
            }
        }
    }

    /// remove the elemnt dom attr
    pub(crate) fn remove_element_dom_attr(element: &Element, attr: &DomAttr) -> Result<(), JsValue> {
        if *VALUE == attr.name {
            DomAttr::set_value_str(element, "");
        } else if *OPEN == attr.name {
            DomAttr::set_open(element, false);
        } else if *CHECKED == attr.name {
            DomAttr::set_checked(element, false);
        } else if *DISABLED == attr.name {
            DomAttr::set_disabled(element, false);
        }
        //actually remove the element
        element.remove_attribute(intern(attr.name))?;

        Ok(())
    }

    /// explicitly call `set_checked` function on the html element
    /// since setting the attribute to false will not unchecked it.
    ///
    /// There are only 2 elements where set_checked is applicable:
    /// - input
    /// - menuitem
    pub(crate) fn set_checked(element: &Element, is_checked: bool) {
        if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
            input.set_checked(is_checked);
        }
    }

    /// explicitly call set_open for details
    /// since setting the attribute `open` to false will not close it.
    ///
    /// TODO: HtmlDialogElement ( but it is not supported on firefox and in safari, only works on chrome)
    ///
    /// Applies to:
    ///  - dialog
    ///  - details
    pub(crate) fn set_open(element: &Element, is_open: bool) {
        if let Some(details) = element.dyn_ref::<HtmlDetailsElement>() {
            details.set_open(is_open);
        }
    }

    /// explicitly call on `set_disabled`
    /// since setting the attribute `disabled` false will not enable it.
    ///
    /// These are 10 elements that we can call `set_disabled` function to.
    /// - input
    /// - button
    /// - textarea
    /// - style
    /// - link
    /// - select
    /// - option
    /// - optgroup
    /// - fieldset
    /// - menuitem
    ///
    /// TODO: use macro to simplify this code
    pub(crate) fn set_disabled(element: &Element, is_disabled: bool) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlStyleElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlLinkElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptGroupElement>() {
            elm.set_disabled(is_disabled);
        } else if let Some(elm) = element.dyn_ref::<HtmlFieldSetElement>() {
            elm.set_disabled(is_disabled);
        }
    }

    /// we explicitly call the `set_value` function in the html element
    ///
    /// TODO: use macro to simplify this code
    pub(crate) fn set_value_str(element: &Element, value: &str) {
        if let Some(elm) = element.dyn_ref::<HtmlInputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlTextAreaElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlSelectElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOptionElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlButtonElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlDataElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlOutputElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlParamElement>() {
            elm.set_value(value);
        }
    }

    /// set the value of this element with an i32 value
    pub(crate) fn set_value_i32(element: &Element, value: i32) {
        if let Some(elm) = element.dyn_ref::<HtmlLiElement>() {
            elm.set_value(value);
        }
    }

    /// set the value of this element with an f64 value
    pub(crate) fn set_value_f64(element: &Element, value: f64) {
        if let Some(elm) = element.dyn_ref::<HtmlMeterElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlProgressElement>() {
            elm.set_value(value);
        }
    }

    /// set the element attribute value with the first numerical value found in values
    pub(crate) fn set_numeric_values(element: &Element, values: &[Value]) {
        let value_i32 = values.first().and_then(|v| v.as_i32());

        let value_f64 = values.first().and_then(|v| v.as_f64());

        if let Some(value_i32) = value_i32 {
            Self::set_value_i32(element, value_i32);
        }
        if let Some(value_f64) = value_f64 {
            Self::set_value_f64(element, value_f64);
        }
    }
}

impl DomAttrValue {
    /// return the value if it is a Simple variant
    pub fn get_simple(&self) -> Option<&Value> {
        match self {
            Self::Simple(v) => Some(v),
            _ => None,
        }
    }

    /// make a string representation of this value if it is a simple value
    pub fn get_string(&self) -> Option<String> {
        let simple = self.get_simple()?;
        Some(simple.to_string())
    }

    /// return event clousre if it an event listener variant
    pub(crate) fn as_event_closure(self) -> Option<Closure<dyn FnMut(web_sys::Event)>> {
        match self {
            Self::EventListener(cb) => Some(cb),
            _ => None,
        }
    }

    fn convert_attr_value_except_listener<MSG>(
        attr_value: &AttributeValue<MSG>,
    ) -> Option<DomAttrValue> {
        match attr_value {
            AttributeValue::FunctionCall(v) => Some(DomAttrValue::FunctionCall(v.clone())),
            AttributeValue::Simple(v) => Some(DomAttrValue::Simple(v.clone())),
            AttributeValue::Style(v) => Some(DomAttrValue::Style(v.clone())),
            AttributeValue::EventListener(_v) => None,
            AttributeValue::Empty => None,
        }
    }
}
