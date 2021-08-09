//! Create html [attributes][0]
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
use crate::Attribute;
use crate::Event;
pub use attribute_macros::*;
pub use attribute_value::AttributeValue;
pub use callback::Callback;
pub use special::{key, replace, skip, Special};
pub use style::Style;
pub use style_macro::*;
pub use value::Value;

#[macro_use]
mod attribute_macros;
#[macro_use]
mod style_macro;
mod attribute_value;
mod callback;
mod special;
mod style;
mod value;

/// create a style attribute
/// ```rust
/// use sauron::prelude::*;
/// use sauron::html::attributes::style;
///
/// let flex:Attribute<()> = style("display", "flex");
/// ```
pub fn style<V, MSG>(style_name: &'static str, value: V) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
{
    mt_dom::attr(
        "style",
        AttributeValue::from_styles(vec![Style::new(style_name, value.into())]),
    )
}

/// A helper function which creates a style attribute by assembling the tuples into a string for the style value.
/// # Example
/// ```rust
/// use sauron::prelude::*;
///
/// let html:Node<()> = div(vec![styles([("display", "flex"), ("flex-direction", "row")])], vec![]);
/// ```
/// is the same way of writing
/// ```rust
/// use sauron::prelude::*;
///
/// let html: Node<()> = div(vec![style!{"display":"flex","flex-direction":"row"}],vec![]);
/// ```
pub fn styles<V, MSG, P>(pairs: P) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V)]>,
{
    let mut styles = vec![];
    for (key, value) in pairs.as_ref() {
        styles.push(Style::new(*key, Into::<Value>::into(value.clone())));
    }
    mt_dom::attr("style", AttributeValue::from_styles(styles))
}

/// A helper function to build styles by accepting pairs
pub fn styles_values<MSG, P>(pairs: P) -> Attribute<MSG>
where
    P: AsRef<[(&'static str, Value)]>,
{
    let mut styles = vec![];
    for (key, value) in pairs.as_ref() {
        styles.push(Style::new(*key, value.clone()));
    }
    mt_dom::attr("style", AttributeValue::from_styles(styles))
}

/// A helper function which creates a style attribute by assembling only the parts that passed the
/// boolean flag
/// ```ignore
///    styles_flag([
///        ("display", "block", self.is_active),
///        ("display", "none", !self.is_active),
///    ]),
/// ```
/// This could also be written as
/// ```ignore
///     styles([("display", if self.is_active { "block" }else{ "none" })])
/// ```
pub fn styles_flag<V, MSG, P>(trio: P) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V, bool)]>,
{
    let mut styles = vec![];
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            styles.push(Style::new(*key, Into::<Value>::into(value.clone())));
        }
    }
    mt_dom::attr("style", AttributeValue::from_styles(styles))
}

/// ```ignore
///    classes_flag([
///        ("dashed", self.is_hidden),
///        ("error", self.has_error),
///    ]),
/// ```
pub fn classes_flag<P, S, MSG>(pair: P) -> Attribute<MSG>
where
    P: AsRef<[(S, bool)]>,
    S: ToString,
{
    let mut class_list = Vec::with_capacity(pair.as_ref().len());
    for (class, flag) in pair.as_ref() {
        if *flag {
            class_list.push(class.to_string());
        }
    }
    classes(class_list)
}

/// a helper function to add multiple classes to a node
///
/// ```ignore
///    div(vec![classes(["dashed", "error"])], vec![])
/// ```
pub fn classes<C, V, MSG>(class_list: C) -> Attribute<MSG>
where
    V: ToString,
    C: AsRef<[V]>,
{
    let class_values: Vec<AttributeValue<MSG>> = class_list
        .as_ref()
        .iter()
        .map(|v| AttributeValue::from_value(Value::from(v.to_string())))
        .collect();
    Attribute::with_multiple_values(None, "class", class_values)
}

/// A helper function for setting attributes with no values such as checked
/// in checkbox input type
/// This is best called to be appended to the node since this
/// returns an array of attributes which doesn't play well with the others
/// Example:
/// ```ignore
/// input(vec![r#type("checkbox")], vec![]).attributes(attrs_flag(vec![(
///                             "checked",
///                             "checked",
///                             is_checked,
///                         )])),
/// ```
pub fn attrs_flag<V, MSG, P>(trio: P) -> Vec<Attribute<MSG>>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V, bool)]>,
{
    let mut attributes: Vec<Attribute<MSG>> =
        Vec::with_capacity(trio.as_ref().len());
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            attributes.push(attr(key, value.clone()));
        }
    }
    attributes
}

/// set the checked value, used checkbox and radio buttons
pub fn checked<MSG>(is_checked: bool) -> Attribute<MSG> {
    if is_checked {
        #[cfg(not(feature = "with-dom"))]
        {
            attr("checked", "checked")
        }
        #[cfg(feature = "with-dom")]
        {
            attr("checked", true)
        }
    } else {
        empty_attr()
    }
}

/// set whether an element is disabled or not
pub fn disabled<MSG>(is_disabled: bool) -> Attribute<MSG> {
    if is_disabled {
        attr("disabled", true)
    } else {
        empty_attr()
    }
}

/// set the inner html of this element without comparing in the diff
/// this always sets the value
/// This is for optimization purposes
/// and will lead to some hacks in the implementation
pub fn inner_html<V, MSG>(inner_html: V) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
{
    mt_dom::attr(
        "inner_html",
        AttributeValue::function_call(inner_html.into()),
    )
}

/// focus the html element
pub fn focus<MSG>(is_focus: bool) -> Attribute<MSG> {
    attr("focus", is_focus)
}

/// a utility function to convert simple value into attribute
pub fn attr<MSG, V: Into<Value>>(att: &'static str, v: V) -> Attribute<MSG> {
    mt_dom::attr(att, AttributeValue::from_value(v.into()))
}

/// a utility function to return create an empty attr, useful for cases where branch expression
/// need to return an attribute which otherwise it can not produce
/// example:
/// ```rust
/// use sauron_core::html::attributes::{title,empty_attr};
/// use sauron_core::Attribute;
///
/// let img_title = Some("this is the image");
/// let result: Attribute<()> = if let Some(img_title) = img_title{
///     title(img_title)
/// }
/// else{
///     empty_attr()
/// };
/// assert_eq!(title("this is the image"), result);
/// ```
pub fn empty_attr<MSG>() -> Attribute<MSG> {
    mt_dom::attr("", AttributeValue::Empty)
}

/// merge the plain values
pub(crate) fn merge_plain_attributes_values<MSG>(
    attr_values: &[&AttributeValue<MSG>],
) -> Option<String> {
    let plain_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Simple(simple) => Some(simple.to_string()),
            AttributeValue::FunctionCall(fvalue) => Some(fvalue.to_string()),
            _ => None,
        })
        .collect();
    if !plain_values.is_empty() {
        Some(plain_values.join(" "))
    } else {
        None
    }
}

/// merge the styles
pub(crate) fn merge_styles_attributes_values<MSG>(
    attr_values: &[&AttributeValue<MSG>],
) -> Option<String> {
    use std::fmt::Write;

    let styles_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Style(styles) => {
                let mut style_str = String::new();
                styles.iter().for_each(|s| {
                    write!(style_str, "{};", s).expect("must write")
                });
                Some(style_str)
            }
            _ => None,
        })
        .collect();

    if !styles_values.is_empty() {
        Some(styles_values.join(" "))
    } else {
        None
    }
}

/// returns (callbacks, plain_attribtues, function_calls)
pub(crate) fn partition_callbacks_from_plain_styles_and_func_calls<MSG>(
    attr: &Attribute<MSG>,
) -> (
    Vec<&Callback<Event, MSG>>,
    Vec<&AttributeValue<MSG>>,
    Vec<&AttributeValue<MSG>>,
    Vec<&AttributeValue<MSG>>,
) {
    let mut callbacks = vec![];
    let mut plain_values = vec![];
    let mut styles = vec![];
    let mut func_values = vec![];
    for av in attr.value() {
        match av {
            AttributeValue::Simple(_plain) => {
                plain_values.push(av);
            }
            AttributeValue::FunctionCall(_call) => {
                func_values.push(av);
            }
            AttributeValue::Style(_) => {
                styles.push(av);
            }
            AttributeValue::EventListener(cb) => {
                callbacks.push(cb);
            }
            _ => (),
        }
    }
    (callbacks, plain_values, styles, func_values)
}
