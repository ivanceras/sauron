use crate::{
    Attribute,
    Node,
};
pub use sauron_vdom::builder::{
    attr,
    on,
    text,
};

#[macro_use]
pub mod attributes;
pub mod events;
pub mod units;

#[inline]
pub fn html_element<MSG>(
    tag: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: Clone,
{
    sauron_vdom::builder::element(tag, attrs, children)
}

#[inline]
pub fn html_element_ns<MSG>(
    tag: &'static str,
    namespace: &'static str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Node<MSG>>,
) -> Node<MSG>
where
    MSG: Clone,
{
    sauron_vdom::builder::element_ns(tag, Some(namespace), attrs, children)
}

macro_rules! declare_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<MSG>(attrs: Vec<$crate::Attribute<MSG>>, children: Vec<$crate::Node<MSG>>) -> $crate::Node<MSG>
                where
                      MSG: Clone,
                {
                    $crate::html::html_element(stringify!($name), attrs, children)
                }
         )*
    }
}

// Organized in the same order as
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
//
// Does not include obsolete elements.
declare_tags! {
    base;
    head;
    link;
    meta;
    body;
    address;
    article;
    aside;
    footer;
    header;
    h1;
    h2;
    h3;
    h4;
    h5;
    h6;
    hgroup;
    main;
    nav;
    section;
    blockquote;
    dd;
    div;
    dl;
    dt;
    figcaption;
    figure;
    hr;
    li;
    ol;
    p;
    pre;
    ul;
    a;
    abbr;
    b;
    bdi;
    bdo;
    br;
    cite;
    code;
    data;
    dfn;
    em;
    i;
    kbd;
    mark;
    q;
    rb;
    rp;
    rt;
    rtc;
    ruby;
    s;
    samp;
    small;
    span;
    strong;
    sub;
    sup;
    time;
    u;
    var;
    wbr;
    area;
    audio;
    img;
    map;
    track;
    video;
    embed;
    iframe;
    object;
    param;
    picture;
    source;
    canvas;
    noscript;
    script;
    del;
    ins;
    caption;
    col;
    colgroup;
    table;
    tbody;
    td;
    tfoot;
    th;
    thead;
    tr;
    button;
    datalist;
    fieldset;
    form;
    input;
    label;
    legend;
    meter;
    optgroup;
    option;
    output;
    progress;
    select;
    textarea;
    details;
    dialog;
    menu;
    menuitem;
    summary;
    template;
}

/// A help function which render the view when the condition is met, otherwise
/// just display a text("")
pub fn view_if<MSG>(flag: bool, node: Node<MSG>) -> Node<MSG>
where
    MSG: Clone,
{
    if flag {
        node
    } else {
        text("")
    }
}

