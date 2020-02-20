use crate::Attribute;
use sauron_vdom::{
    builder::attr,
    Value,
};

#[macro_export]
macro_rules! declare_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<V, MSG>(v: V) -> crate::Attribute<MSG>
                where V: Into<Value>,
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
            #[allow(non_snake_case)]
            pub fn $name<V, MSG>(v: V) -> crate::Attribute<MSG>
                where V: Into<Value>,
                {
                    attr($attribute, v)
                }
         )*
    }
}

/// declare html attributes, at the same time this also
/// fills up the HTML_ATTRS const with all the common html attributes
macro_rules! declare_html_attributes{
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_attributes!{ $($name;)*}
        pub(crate) const HTML_ATTRS:[&'static str; 119] = [$(stringify!($name),)*];
    }
}

/// declare html attributes, at the same time this also
/// fills up the HTML_ATTRS_SPECIAL const with the html attribute that are not
/// regular identifiers
macro_rules! declare_html_attributes_special{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        declare_attributes!{ $($name;)*}
        pub(crate) const HTML_ATTRS_SPECIAL:[(&'static str,&'static str); 12] = [$((stringify!($name),$attribute),)*];
    }
}

// List from html attributes
// https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
declare_html_attributes! {
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
    key;
}

// attributes with dash
declare_html_attributes_special! {
    accept_charset => "accept-charset";

    r#async => "async";
    async_ => "async";

    r#for => "for";
    for_ => "for";

    font_family => "font-family";
    font_size => "font-size";
    flex_direction => "flex-direction";

    r#loop => "loop";
    loop_ => "loop";

    r#type => "type";
    type_ => "type";
}
