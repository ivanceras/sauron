//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
pub use sauron_vdom::builder::attr;
use sauron_vdom::builder::Attribute;
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
            pub fn $name<'a, V,CB>(v: V) -> Attribute<'a,CB>
                where V: Into<Value>,
                    CB: Clone,
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
            pub fn $name<'a, V,CB>(v: V) -> Attribute<'a,CB>
                where V: Into<Value>,
                    CB: Clone,
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
///div([style("display:flex;flex-direction:row;")],[])
/// ```
pub fn styles<'a, V, MSG, P>(pairs: P) -> Attribute<'a, MSG>
where
    V: Into<Value> + Clone,
    MSG: Clone,
    P: AsRef<[(&'static str, V)]>,
{
    let mut style_str = String::new();
    for (key, value) in pairs.as_ref() {
        let value: Value = value.clone().into();
        style_str.push_str(&format!("{}:{};", key, value.to_string()));
    }
    style(style_str)
}

#[cfg(test)]
mod test {
    use crate::html::attributes::*;
    use crate::html::*;
    #[test]
    fn test_styles() {
        let actual: Node<&'static str> = div(
            [styles([("display", "flex"), ("flex-direction", "row")])],
            [],
        );
        let expected = div([style("display:flex;flex-direction:row;")], []);
        assert_eq!(actual, expected);
    }
}
