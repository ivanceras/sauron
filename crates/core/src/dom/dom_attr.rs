use crate::vdom::Value;
use crate::vdom::Style;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::Element;
use wasm_bindgen::intern;
use crate::vdom::AttributeName;
use crate::vdom::Namespace;
use web_sys::{
    self, HtmlButtonElement, HtmlDataElement, HtmlDetailsElement, HtmlFieldSetElement,
    HtmlInputElement, HtmlLiElement, HtmlLinkElement, HtmlMeterElement, HtmlOptGroupElement,
    HtmlOptionElement, HtmlOutputElement, HtmlParamElement, HtmlProgressElement, HtmlSelectElement,
    HtmlStyleElement, HtmlTextAreaElement, Node, Text,
};
 
 use crate::vdom::AttributeValue;
use crate::vdom::Attribute;

pub struct DomAttr {
    pub namespace: Option<&'static str>,
    pub name: &'static str,
    pub value: Vec<DomAttrValue>,
}

pub enum DomAttrValue {
    FunctionCall(Value),
    Simple(Value),
    Style(Vec<Style>),
    EventListener(Closure<dyn FnMut(web_sys::Event)>),
}

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
    pub fn set_element_dom_attr_except_listeners(element: &Element, attr: DomAttr) {
        let attr_name = intern(attr.name);
        let attr_namespace = attr.namespace;

        let GroupedDomAttrValues {
            listeners,
            plain_values,
            styles,
            function_calls,
        } = attr.group_values();
        Self::set_element_style(element, attr_name, styles);
        Self::set_element_function_call_values(element, attr_name, function_calls);
        Self::set_element_simple_values(element, attr_name, attr_namespace, plain_values);
    }

    pub fn set_element_style(element: &Element, attr_name: AttributeName, styles: Vec<Style>) {
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

    // do function calls such as set_inner_html
    pub fn set_element_function_call_values(element: &Element, attr_name: AttributeName, function_calls: Vec<Value>) {
        if let Some(merged_func_values) = Value::merge_to_string(function_calls.iter()) {
            if attr_name == "inner_html" {
                element.set_inner_html(&merged_func_values);
            }
        }
    }

    // set simple values
    pub fn set_element_simple_values(element: &Element, 
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
                match attr_name {
                    "value" => {
                        element
                            .set_attribute(attr_name, &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_value_str(element, &merged_plain_values);
                        Self::set_numeric_values(element, &plain_values);
                    }
                    "open" => {
                        let is_open: bool = plain_values
                            .first()
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);

                        element
                            .set_attribute(attr_name, &is_open.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_open(element, is_open);
                    }
                    "checked" => {
                        let is_checked: bool = plain_values
                            .first()
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);

                        element
                            .set_attribute(attr_name, &is_checked.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_checked(element, is_checked)
                    }
                    "disabled" => {
                        let is_disabled: bool = plain_values
                            .first()
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);

                        element
                            .set_attribute(attr_name, &is_disabled.to_string())
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                        Self::set_disabled(element, is_disabled);
                    }
                    _ => {
                        element
                            .set_attribute(attr_name, &merged_plain_values)
                            .unwrap_or_else(|_| {
                                panic!("Error setting an attribute for {element:?}")
                            });
                    }
                }
            }
        }
    }

    /// explicitly call `set_checked` function on the html element
    /// since setting the attribute to false will not unchecked it.
    ///
    /// There are only 2 elements where set_checked is applicable:
    /// - input
    /// - menuitem
    pub fn set_checked(element: &Element, is_checked: bool) {
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
    pub fn set_open(element: &Element, is_open: bool) {
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
    pub fn set_disabled(element: &Element, is_disabled: bool) {
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
    pub fn set_value_str(element: &Element, value: &str) {
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

    pub fn set_value_i32(element: &Element, value: i32) {
        if let Some(elm) = element.dyn_ref::<HtmlLiElement>() {
            elm.set_value(value);
        }
    }

    pub fn set_value_f64(element: &Element, value: f64) {
        if let Some(elm) = element.dyn_ref::<HtmlMeterElement>() {
            elm.set_value(value);
        } else if let Some(elm) = element.dyn_ref::<HtmlProgressElement>() {
            elm.set_value(value);
        }
    }

    /// set the element attribute value with the first numerical value found in values
    pub fn set_numeric_values(element: &Element, values: &[Value]) {
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

    pub fn as_event_closure(self) -> Option<Closure<dyn FnMut(web_sys::Event)>> {
        match self {
            Self::EventListener(cb) => Some(cb),
            _ => None,
        }
    }

    fn convert_attr_value_except_listener<MSG>(attr_value: &AttributeValue<MSG>) -> Option<DomAttrValue> {
        match attr_value {
            AttributeValue::FunctionCall(v) => Some(DomAttrValue::FunctionCall(v.clone())),
            AttributeValue::Simple(v) => Some(DomAttrValue::Simple(v.clone())),
            AttributeValue::Style(v) => Some(DomAttrValue::Style(v.clone())),
            AttributeValue::EventListener(_v) => None,
            AttributeValue::Empty => None,
        }
    }

}

