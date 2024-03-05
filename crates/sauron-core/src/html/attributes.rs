//! Create html [attributes][0]
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
use std::borrow::Cow;
use crate::vdom;
use crate::vdom::AttributeValue;
use crate::vdom::Value;

pub use crate::{dom::Event, vdom::Attribute};
pub use attribute_macros::commons::*;
pub use attribute_macros::*;
pub use listener::Listener;
pub use special::{key, replace, skip, skip_criteria};
pub use style::Style;

#[macro_use]
mod attribute_macros;
mod listener;
mod special;
mod style;

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
    let styles = pairs
        .into_iter()
        .map(|(key, value)| Style::new(key, value));
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
pub fn classes_flag<MSG>(pair: impl IntoIterator<Item = (impl Into<Value>, bool)>) -> Attribute<MSG> {
    let class_list = pair.into_iter().filter_map(
        |(class, flag)| {
            if flag {
                Some(class.into())
            } else {
                None
            }
        },
    );

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

/// set the inner html of this element without comparing in the diff
/// this always sets the value
/// This is for optimization purposes
/// and will lead to some hacks in the implementation
/// # Examples
/// ```rust
/// use sauron::{*, html::{*, attributes::*}};
///
/// let html:Node<()> =
///     div(vec![inner_html("<p>This is a paragraph <b>injected</b> into a <strong>div</strong> via <i>inner_html</i></p>")], vec![]);
/// ```
pub fn inner_html<MSG>(inner_html: impl Into<Value>) -> Attribute<MSG> {
    vdom::attr(
        "inner_html",
        AttributeValue::function_call(inner_html.into()),
    )
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

/// merge the plain values
#[doc(hidden)]
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
#[doc(hidden)]
pub(crate) fn merge_styles_attributes_values<MSG>(
    attr_values: &[&AttributeValue<MSG>],
) -> Option<String> {
    use std::fmt::Write;

    let styles_values: Vec<String> = attr_values
        .iter()
        .flat_map(|att_value| match att_value {
            AttributeValue::Style(styles) => {
                let mut style_str = String::new();
                styles
                    .iter()
                    .for_each(|s| write!(style_str, "{s};").expect("must write"));
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

/// The Attributes partition into 4 different types
pub struct SegregatedAttributes<'a, MSG> {
    /// the listeners of the event listeners
    pub listeners: Vec<&'a Listener<Event, MSG>>,
    /// plain attribute values
    pub plain_values: Vec<&'a AttributeValue<MSG>>,
    /// style attribute values
    pub styles: Vec<&'a AttributeValue<MSG>>,
    /// function calls
    pub function_calls: Vec<&'a AttributeValue<MSG>>,
}

/// returns (listeners, plain_attribtues, function_calls)
#[doc(hidden)]
pub(crate) fn partition_callbacks_from_plain_styles_and_func_calls<MSG>(
    attr: &Attribute<MSG>,
) -> SegregatedAttributes<MSG> {
    let mut listeners = vec![];
    let mut plain_values = vec![];
    let mut styles = vec![];
    let mut function_calls = vec![];
    for av in attr.value() {
        match av {
            AttributeValue::Simple(_plain) => {
                plain_values.push(av);
            }
            AttributeValue::FunctionCall(_call) => {
                function_calls.push(av);
            }
            AttributeValue::Style(_) => {
                styles.push(av);
            }
            AttributeValue::EventListener(cb) => {
                listeners.push(cb);
            }
            _ => (),
        }
    }
    SegregatedAttributes {
        listeners,
        plain_values,
        styles,
        function_calls,
    }
}
