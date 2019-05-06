//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use crate::Attribute;
pub use sauron_vdom::builder::attr;
use sauron_vdom::Value;

macro_rules! declare_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<V, MSG>(v: V) -> crate::Attribute<MSG>
                where V: Into<Value>,
                    MSG: Clone,
                {
                    attr(stringify!($name), v)
                }
         )*
    };
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<V, MSG>(v: V) -> crate::Attribute<MSG>
                where V: Into<Value>,
                    MSG: Clone,
                {
                    attr($attribute, v)
                }
         )*
    }
}

// List from html attributes
// https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
declare_attributes! {
    accept;
    accesskey;
    action;
    align;
    allow;
    alt;
    autocapitalize;
    autocomplete;
    autofocus;
    autoplay;
    background;
    bgcolor;
    border;
    buffered;
    challenge;
    charset;
    checked;
    cite;
    class;
    code;
    codebase;
    color;
    cols;
    colspan;
    content;
    contenteditable;
    contextmenu;
    controls;
    coords;
    crossorigin;
    csp;
    data;
    datetime;
    decoding;
    default;
    defer;
    dir;
    dirname;
    disabled;
    download;
    draggable;
    dropzone;
    enctype;
    enterkeyhint;
    form;
    formaction;
    formnovalidate;
    headers;
    height;
    hidden;
    high;
    href;
    hreflang;
    http;
    icon;
    id;
    importance;
    integrity;
    intrinsicsize;
    inputmode;
    ismap;
    itemprop;
    keytype;
    kind;
    lang;
    language;
    loading;
    list;
    low;
    manifest;
    max;
    maxlength;
    minlength;
    media;
    method;
    min;
    multiple;
    muted;
    name;
    novalidate;
    open;
    optimum;
    pattern;
    ping;
    placeholder;
    poster;
    preload;
    radiogroup;
    readonly;
    referrerpolicy;
    rel;
    required;
    reversed;
    rows;
    rowspan;
    sandbox;
    scope;
    scoped;
    selected;
    shape;
    size;
    sizes;
    slot;
    spellcheck;
    src;
    srcdoc;
    srclang;
    srcset;
    start;
    step;
    style;
    summary;
    tabindex;
    target;
    title;
    translate;
    usemap;
    value;
    width;
    wrap;
}

declare_attributes! {
    key;
}

// attributes with dash
declare_attributes! {
    accept_charset => "accept-charset";

    r#async => "async";

    r#for => "for";
    font_family => "font-family";

    font_size => "font-size";

    flex_direction => "flex-direction";
    r#loop => "loop";
    r#type => "type";
}

/// A helper function which creates a style attribute by assembling the tuples into a string for the style value.
/// ```ignore
///  div([styles([("display", "flex"), ("flex-direction", "row")])], [])
/// ```
/// is the same way of writing
/// ```ignore
/// div([style("display:flex;flex-direction:row;")],[])
/// ```
pub fn styles<V, MSG, P>(pairs: P) -> Attribute<MSG>
    where V: Into<Value> + Clone,
          MSG: Clone,
          P: AsRef<[(&'static str, V)]>
{
    let mut style_str = String::new();
    for (key, value) in pairs.as_ref() {
        let value: Value = value.clone().into();
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
    where V: Into<Value> + Clone,
          MSG: Clone,
          P: AsRef<[(&'static str, V, bool)]>
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

pub fn classes_flag<P, MSG>(pair: P) -> Attribute<MSG>
    where P: AsRef<[(&'static str, bool)]>,
          MSG: Clone
{
    let mut class_list = Vec::with_capacity(pair.as_ref().len());
    for (class, flag) in pair.as_ref() {
        if *flag {
            class_list.push(class.to_string());
        }
    }
    class(class_list.join(" "))
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
    where V: Into<Value> + Clone,
          MSG: Clone,
          P: AsRef<[(&'static str, V, bool)]>
{
    let mut attributes = Vec::with_capacity(trio.as_ref().len());
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            attributes.push(attr(key, value.clone()));
        }
    }
    attributes
}

/// a helper function which append `px` into a value
/// Example:
/// ```ignore
/// style("width", px(100))
/// ```
pub fn px<V>(v: V) -> String
    where V: Into<Value> + Clone
{
    format!("{}px", v.into())
}
