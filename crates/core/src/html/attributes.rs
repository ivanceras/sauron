//! Create html [attributes][0]
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
use crate::vdom;
use crate::vdom::AttributeValue;
use crate::vdom::Value;
use std::borrow::Cow;

pub use crate::vdom::EventCallback;
pub use crate::vdom::Style;
pub use crate::vdom::{key, replace, skip, skip_criteria};
pub use crate::{dom::Event, vdom::Attribute};
pub use attribute_macros::commons::*;
pub use attribute_macros::*;

#[macro_use]
mod attribute_macros;

/// A helper function which creates a style attribute by assembling the tuples into a string for the style value.
/// # Example
/// ```rust
/// use sauron::{*, html::attributes::styles};
///
/// let html:Node<()> = div(vec![styles([("display", "flex"), ("flex-direction", "row")])], vec![]);
/// ```
/// is the same way of writing
/// ```rust
/// use sauron::*;
/// use sauron::html::attributes::styles;
///
/// let html: Node<()> = div(vec![style!{"display":"flex","flex-direction":"row"}],vec![]);
/// ```
pub fn styles<MSG>(
    pairs: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Value>)>,
) -> Attribute<MSG> {
    let styles = pairs
        .into_iter()
        .map(|(key, value)| Style::new(key, Into::<Value>::into(value)));
    vdom::attr("style", AttributeValue::from_styles(styles))
}

/// A helper function to build styles by accepting pairs
pub fn styles_values<MSG>(
    pairs: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Value>)>,
) -> Attribute<MSG> {
    let styles = pairs.into_iter().map(|(key, value)| Style::new(key, value));
    vdom::attr("style", AttributeValue::from_styles(styles))
}

/// A helper function which creates a style attribute by assembling only the parts that passed the
/// boolean flag.
/// # Examples
/// ```rust
/// use sauron::*;
///
/// let is_active = true;
/// let display:Attribute<()> = styles_flag([
///         ("display", "block", is_active),
///         ("display", "none", !is_active),
///     ]);
/// ```
/// This could also be written as
/// ```rust
/// use sauron::{*, html::attributes::styles};
///
/// let is_active = true;
/// let display:Attribute<()> =
///     styles([("display", if is_active { "block" }else{ "none" })]);
/// ```
pub fn styles_flag<MSG>(
    trio: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Value>, bool)>,
) -> Attribute<MSG> {
    let styles = trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(Style::new(key, value))
        } else {
            None
        }
    });
    vdom::attr("style", AttributeValue::from_styles(styles))
}

/// A helper function which takes an array of tuple of class and a flag. The final class is
/// assembled using only the values that has a flag which evaluates to true.
/// # Examples
/// ```rust
/// use sauron::*;
/// let is_hidden = true;
/// let has_error = true;
///
/// let line:Attribute<()> = classes_flag([
///        ("dashed", is_hidden),
///        ("error", has_error),
///    ]);
/// ```
pub fn classes_flag<MSG>(
    pair: impl IntoIterator<Item = (impl Into<Value>, bool)>,
) -> Attribute<MSG> {
    let class_list = pair
        .into_iter()
        .filter_map(|(class, flag)| if flag { Some(class.into()) } else { None });

    classes(class_list)
}

/// a helper function to add multiple classes to a node
/// # Examples
///
/// ```rust
/// use sauron::{*,html::attributes::classes};
///
/// let html: Node<()> =
///    div(vec![classes(["dashed", "error"])], vec![]);
/// ```
pub fn classes<MSG>(class_list: impl IntoIterator<Item = impl Into<Value>>) -> Attribute<MSG> {
    let class_values = class_list
        .into_iter()
        .map(|v| AttributeValue::from(v.into()));

    Attribute::with_multiple_values(None, "class", class_values)
}

/// A helper function for setting attributes with no values such as checked
/// in checkbox input type
/// This is best called to be appended to the node since this
/// returns an array of attributes which doesn't play well with the others
/// # Examples
/// ```rust
/// use sauron::{*,html::*, html::attributes::attrs_flag};
///
/// let is_checked = true;
/// let html: Node<()> =
///     input(vec![r#type("checkbox")], vec![]).with_attributes(attrs_flag(vec![(
///                             "checked",
///                             "checked",
///                             is_checked,
///                         )]));
/// ```
pub fn attrs_flag<MSG>(
    trio: impl IntoIterator<Item = (&'static str, impl Into<Value>, bool)>,
) -> impl IntoIterator<Item = Attribute<MSG>> {
    trio.into_iter().filter_map(|(key, value, flag)| {
        if flag {
            Some(attr(key, value.into()))
        } else {
            None
        }
    })
}

/// Set the attribute of this element if value is Some, empty attribute otherwise
/// # Examples
/// ```rust
/// use sauron::{*, html::attributes::maybe_attr};
///
/// let width = Some(10);
/// let html: Node<()> = button(vec![maybe_attr("width", width)], vec![]);
/// let expected = r#"<button width="10"></button>"#;
/// assert_eq!(expected, html.render_to_string());
///
/// let width = None::<usize>;
/// let html: Node<()> = button(vec![maybe_attr("width", width)], vec![]);
/// let expected = r#"<button></button>"#;
/// assert_eq!(expected, html.render_to_string());
/// ```
pub fn maybe_attr<MSG>(
    name: vdom::AttributeName,
    value: Option<impl Into<Value>>,
) -> Attribute<MSG> {
    if let Some(value) = value {
        attr(name, value)
    } else {
        empty_attr()
    }
}

/// set the checked value, used checkbox and radio buttons
/// # Examples
/// ```rust
/// use sauron::*;
///
/// let html: Node<()> =
///     input(vec![r#type("checkbox"), checked(true)], vec![]);
/// ```
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
/// # Examples
/// ```rust
/// use sauron::{*, html::*, html::attributes::*};
///
/// let html: Node<()> =
///     input(vec![r#type("checkbox"), disabled(true)], vec![]);
/// ```
pub fn disabled<MSG>(is_disabled: bool) -> Attribute<MSG> {
    if is_disabled {
        attr("disabled", true)
    } else {
        empty_attr()
    }
}

/// set whether an element, ie: details, that is the contents of the
/// details are currently visible
pub fn open<MSG>(is_open: bool) -> Attribute<MSG> {
    if is_open {
        attr("open", true)
    } else {
        empty_attr()
    }
}

/// focus the html element
/// # Examples
/// ```rust
/// use sauron::{*, html::*, html::attributes::*};
///
/// let editor:Node<()> = textarea(vec![focus(true)], vec![]);
/// ```
pub fn focus<MSG>(is_focus: bool) -> Attribute<MSG> {
    attr("focus", is_focus)
}

/// a utility function to convert simple value into attribute
/// # Examples
/// ```rust
/// use sauron::{*,html::attributes::attr};
///
/// let data_id: Attribute<()> = attr("data-id", 42);
/// ```
pub fn attr<MSG>(att: &'static str, v: impl Into<Value>) -> Attribute<MSG> {
    vdom::attr(att, AttributeValue::from(v.into()))
}

/// a utility function to return create an empty attr, useful for cases where branch expression
/// need to return an attribute which otherwise it can not produce
/// example:
/// ```rust
/// use sauron::*;
/// use sauron::html::attributes::empty_attr;
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
    vdom::attr("", AttributeValue::Empty)
}
