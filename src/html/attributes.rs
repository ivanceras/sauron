//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use crate::Attribute;
pub use attribute_macros::*;
pub use sauron_vdom::builder::attr;
use sauron_vdom::AttribValue;
use sauron_vdom::Value;

#[macro_use]
mod attribute_macros;
#[macro_use]
mod style_macro;

/// A helper function which creates a style attribute by assembling the tuples into a string for the style value.
/// ```ignore
///  div([styles([("display", "flex"), ("flex-direction", "row")])], [])
/// ```
/// is the same way of writing
/// ```ignore
/// div([style("display:flex;flex-direction:row;")],[])
/// ```
pub fn styles<'a, V, MSG, P>(pairs: P) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'a str, V)]>,
{
    let mut style_str = String::new();
    for (key, value) in pairs.as_ref() {
        let value: Value = value.clone().into();
        style_str.push_str(&format!("{}:{};", key, value.to_string()));
    }
    style(style_str)
}

pub fn styles_values<'a, MSG, P>(pairs: P) -> Attribute<MSG>
where
    P: AsRef<[(&'a str, Value)]>,
{
    let mut style_str = String::new();
    for (key, value) in pairs.as_ref() {
        style_str.push_str(&format!("{}:{};", key, value.to_string()));
    }
    style(style_str)
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
    let mut style_list = Vec::with_capacity(trio.as_ref().len());
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            let value: Value = value.clone().into();
            style_list.push(format!("{}:{};", key, value.to_string()));
        }
    }
    style(style_list.join(""))
}

/// ```ignore
///    classes_flag([
///        ("dashed", self.is_hidden),
///        ("error", self.has_error),
///    ]),
/// ```
pub fn classes_flag<P, MSG>(pair: P) -> Attribute<MSG>
where
    P: AsRef<[(&'static str, bool)]>,
{
    let mut class_list = Vec::with_capacity(pair.as_ref().len());
    for (class, flag) in pair.as_ref() {
        if *flag {
            class_list.push((*class).to_string());
        }
    }
    class(class_list.join(" "))
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
    class(
        class_list
            .as_ref()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

/// A helper function for setting attributes with no values such as checked
/// in checkbox input type
/// This is best called to be appended to the node since this
/// returns an array of attributes which doesn't play well with the others
/// Example:
/// ```ignore
/// input([r#type("checkbox")], []).attributes(attrs_flag([(
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

pub fn checked<MSG>(is_checked: bool) -> Attribute<MSG> {
    if is_checked {
        attr("checked", "checked")
    } else {
        attr("", "")
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
    Attribute {
        name: "inner_html",
        value: AttribValue::FuncCall(inner_html.into()),
        namespace: None,
    }
}
