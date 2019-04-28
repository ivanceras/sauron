use std::convert::AsRef;

use crate::{Attribute,
            Node};
pub use sauron_vdom::builder::{attr,
                               on,
                               text};

#[macro_use]
pub mod attributes;
pub mod events;

#[inline]
pub fn html_element<A, C, MSG>(tag: &'static str,
                               attrs: A,
                               children: C)
                               -> Node<MSG>
    where C: AsRef<[Node<MSG>]>,
          A: AsRef<[Attribute<MSG>]>,
          MSG: Clone
{
    sauron_vdom::builder::element(tag, attrs, children)
}

#[inline]
pub fn html_element_ns<A, C, MSG>(tag: &'static str,
                                  namespace: &'static str,
                                  attrs: A,
                                  children: C)
                                  -> Node<MSG>
    where C: AsRef<[Node<MSG>]>,
          A: AsRef<[Attribute<MSG>]>,
          MSG: Clone
{
    sauron_vdom::builder::element_ns(tag, namespace, attrs, children)
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
            pub fn $name<A, C,MSG>(attrs: A, children: C) -> $crate::Node<MSG>
                where C: AsRef<[$crate::Node<MSG>]>,
                      A: AsRef<[$crate::Attribute<MSG>]>,
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
    // Document metadata

    /// Build a
    /// [`<base>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
    /// element.
    base;
    /// Build a
    /// [`<head>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head)
    /// element.
    head;
    /// Build a
    /// [`<link>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
    /// element.
    link;
    /// Build a
    /// [`<meta>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
    /// element.
    meta;
    /// Build a
    /// [`<title>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    /// element.
    title;

    // Sectioning root

    /// Build a
    /// [`<body>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    /// element.
    body;

    // Content sectioning

    /// Build a
    /// [`<address>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
    /// element.
    address;
    /// Build a
    /// [`<article>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
    /// element.
    article;
    /// Build a
    /// [`<aside>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
    /// element.
    aside;
    /// Build a
    /// [`<footer>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/footer)
    /// element.
    footer;
    /// Build a
    /// [`<header>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header)
    /// element.
    header;
    /// Build a
    /// [`<h1>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
    /// element.
    h1;
    /// Build a
    /// [`<h2>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
    /// element.
    h2;
    /// Build a
    /// [`<h3>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
    /// element.
    h3;
    /// Build a
    /// [`<h4>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4)
    /// element.
    h4;
    /// Build a
    /// [`<h5>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
    /// element.
    h5;
    /// Build a
    /// [`<h6>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6)
    /// element.
    h6;
    /// Build a
    /// [`<hgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
    /// element.
    hgroup;
    /// Build a
    /// [`<main>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
    /// element.
    main;
    /// Build a
    /// [`<nav>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
    /// element.
    nav;
    /// Build a
    /// [`<section>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
    /// element.
    section;

    // Text content

    /// Build a
    /// [`<blockquote>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
    /// element.
    blockquote;
    /// Build a
    /// [`<dd>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
    /// element.
    dd;
    /// Build a
    /// [`<div>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    /// element.
    div;
    /// Build a
    /// [`<dl>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl)
    /// element.
    dl;
    /// Build a
    /// [`<dt>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
    /// element.
    dt;
    /// Build a
    /// [`<figcaption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
    /// element.
    figcaption;
    /// Build a
    /// [`<figure>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure)
    /// element.
    figure;
    /// Build a
    /// [`<hr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)
    /// element.
    hr;
    /// Build a
    /// [`<li>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
    /// element.
    li;
    /// Build a
    /// [`<ol>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
    /// element.
    ol;
    /// Build a
    /// [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
    /// element.
    p;
    /// Build a
    /// [`<pre>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
    /// element.
    pre;
    /// Build a
    /// [`<ul>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
    /// element.
    ul;

    // Inline text semantics

    /// Build a
    /// [`<a>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
    /// element.
    a;
    /// Build a
    /// [`<abbr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
    /// element.
    abbr;
    /// Build a
    /// [`<b>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
    /// element.
    b;
    /// Build a
    /// [`<bdi>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
    /// element.
    bdi;
    /// Build a
    /// [`<bdo>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
    /// element.
    bdo;
    /// Build a
    /// [`<br>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br)
    /// element.
    br;
    /// Build a
    /// [`<cite>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
    /// element.
    cite;
    /// Build a
    /// [`<code>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
    /// element.
    code;
    /// Build a
    /// [`<data>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
    /// element.
    data;
    /// Build a
    /// [`<dfn>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn)
    /// element.
    dfn;
    /// Build a
    /// [`<em>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
    /// element.
    em;
    /// Build a
    /// [`<i>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
    /// element.
    i;
    /// Build a
    /// [`<kbd>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
    /// element.
    kbd;
    /// Build a
    /// [`<mark>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
    /// element.
    mark;
    /// Build a
    /// [`<q>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
    /// element.
    q;
    /// Build a
    /// [`<rb>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rb)
    /// element.
    rb;
    /// Build a
    /// [`<rp>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp)
    /// element.
    rp;
    /// Build a
    /// [`<rt>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
    /// element.
    rt;
    /// Build a
    /// [`<rtc>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rtc)
    /// element.
    rtc;
    /// Build a
    /// [`<ruby>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
    /// element.
    ruby;
    /// Build a
    /// [`<s>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
    /// element.
    s;
    /// Build a
    /// [`<samp>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
    /// element.
    samp;
    /// Build a
    /// [`<small>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
    /// element.
    small;
    /// Build a
    /// [`<span>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
    /// element.
    span;
    /// Build a
    /// [`<strong>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
    /// element.
    strong;
    /// Build a
    /// [`<sub>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
    /// element.
    sub;
    /// Build a
    /// [`<sup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
    /// element.
    sup;
    /// Build a
    /// [`<time>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
    /// element.
    time;
    /// Build a
    /// [`<u>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
    /// element.
    u;
    /// Build a
    /// [`<var>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
    /// element.
    var;
    /// Build a
    /// [`<wbr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/wbr)
    /// element.
    wbr;

    // Image and multimedia

    /// Build a
    /// [`<area>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
    /// element.
    area;
    /// Build a
    /// [`<audio>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
    /// element.
    audio;
    /// Build a
    /// [`<img>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    /// element.
    img;
    /// Build a
    /// [`<map>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
    /// element.
    map;
    /// Build a
    /// [`<track>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
    /// element.
    track;
    /// Build a
    /// [`<video>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
    /// element.
    video;

    // Embedded content

    /// Build a
    /// [`<embed>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
    /// element.
    embed;
    /// Build a
    /// [`<iframe>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
    /// element.
    iframe;
    /// Build a
    /// [`<object>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
    /// element.
    object;
    /// Build a
    /// [`<param>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/param)
    /// element.
    param;
    /// Build a
    /// [`<picture>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
    /// element.
    picture;
    /// Build a
    /// [`<source>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
    /// element.
    source;

    // Scripting

    /// Build a
    /// [`<canvas>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
    /// element.
    canvas;
    /// Build a
    /// [`<noscript>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
    /// element.
    noscript;
    /// Build a
    /// [`<script>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
    /// element.
    script;

    // Demarcating edits

    /// Build a
    /// [`<del>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
    /// element.
    del;
    /// Build a
    /// [`<ins>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
    /// element.
    ins;

    // Table content

    /// Build a
    /// [`<caption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption)
    /// element.
    caption;
    /// Build a
    /// [`<col>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
    /// element.
    col;
    /// Build a
    /// [`<colgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    /// element.
    colgroup;
    /// Build a
    /// [`<table>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
    /// element.
    table;
    /// Build a
    /// [`<tbody>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
    /// element.
    tbody;
    /// Build a
    /// [`<td>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
    /// element.
    td;
    /// Build a
    /// [`<tfoot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    /// element.
    tfoot;
    /// Build a
    /// [`<th>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
    /// element.
    th;
    /// Build a
    /// [`<thead>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead)
    /// element.
    thead;
    /// Build a
    /// [`<tr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr)
    /// element.
    tr;

    // Forms

    /// Build a
    /// [`<button>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    /// element.
    button;
    /// Build a
    /// [`<datalist>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
    /// element.
    datalist;
    /// Build a
    /// [`<fieldset>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
    /// element.
    fieldset;
    /// Build a
    /// [`<form>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
    /// element.
    form;
    /// Build a
    /// [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    /// element.
    input;
    /// Build a
    /// [`<label>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
    /// element.
    label;
    /// Build a
    /// [`<legend>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/legend)
    /// element.
    legend;
    /// Build a
    /// [`<meter>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
    /// element.
    meter;
    /// Build a
    /// [`<optgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
    /// element.
    optgroup;
    /// Build a
    /// [`<option>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
    /// element.
    option;
    /// Build a
    /// [`<output>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
    /// element.
    output;
    /// Build a
    /// [`<progress>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
    /// element.
    progress;
    /// Build a
    /// [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
    /// element.
    select;
    /// Build a
    /// [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    /// element.
    textarea;

    // Interactive elements

    /// Build a
    /// [`<details>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
    /// element.
    details;
    /// Build a
    /// [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
    /// element.
    dialog;
    /// Build a
    /// [`<menu>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu)
    /// element.
    menu;
    /// Build a
    /// [`<menuitem>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menuitem)
    /// element.
    menuitem;
    /// Build a
    /// [`<summary>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary)
    /// element.
    summary;

    // Web components

    /// Build a
    /// [`<slot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
    /// element.
    slot;
    /// Build a
    /// [`<template>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
    /// element.
    template;
}

/*
#[cfg(test)]
mod tests {
    use crate::{Element,
                Node};
    use maplit::btreemap;
    //TODO: HashMap makes more sense, rewrite this to hashamp
    use crate::html::{attributes::*,
                      events::*,
                      *};
    use sauron_vdom::{builder::{attr,
                                text},
                      Callback,
                      Text};
    use std::collections::BTreeMap;
    use web_sys::Event;

    #[test]
    fn simple_builder() {
        let div: Element<()> =
            Element::new("div").add_attributes([attr("class", "some-class")]);
        let expected: Element<()> = Element { tag: "div",
                                              attrs: btreemap! {
                                                  "class" => "some-class".into(),
                                              },
                                              events: BTreeMap::new(),
                                              children: vec![],
                                              namespace: None };

        assert_eq!(div, expected);
    }

    #[test]
    fn builder_with_event() {
        let cb = |x: Event| {
            println!("hello! {:?}", x);
        };
        let callback: Callback<Event, ()> = cb.into();
        let div: Element<()> =
            Element::new("div").add_event_listener("click", callback.clone());
        let expected: Element<()> = Element { tag: "div",
                                              events: btreemap! {
                                                  "click" => callback.clone(),
                                              },
                                              attrs: BTreeMap::new(),
                                              children: vec![],
                                              namespace: None };

        assert_eq!(div, expected,
                   "Cloning a callback should only clone the reference");
    }

    #[test]
    fn builder_with_children() {
        let div: Element<()> =
            Element::new("div").add_attributes([attr("class", "some-class")])
                               .add_children(vec![Node::Text(
                Text { text: "Hello".to_string(), },
            )]);
        let expected = Element { tag: "div",
                                 attrs: btreemap! {
                                     "class" => "some-class".into(),
                                 },
                                 children: vec![Node::Text(
            Text { text: "Hello".to_string() }
        )],
                                 events: BTreeMap::new(),
                                 namespace: None };

        assert_eq!(div, expected);
    }

    #[test]
    fn div_builder() {
        let clicked = |_| {
            println!("clicked");
        };
        let cb: Callback<Event, ()> = clicked.into();
        let div: Node<()> = div([class("some-class"),
                                 r#type("submit"),
                                 on("click", cb.clone())],
                                [div([class("some-class")],
                                     [text("Hello world!")])]);
        println!("{:#?}", div);
        let expected = Node::Element(Element { tag: "div",
                                               attrs: btreemap! {
                                                  "class" => "some-class".into(),
                                                  "type" => "submit".into(),
                                               },
                                               events: btreemap! {
                                                   "click" => cb.clone(),
                                               },
                                               namespace: None,
                                               children: vec![
            Node::Element(Element { tag: "div",
                                    attrs: btreemap! {
                                        "class" => "some-class".into()
                                    },
                                    children: vec![Node::Text(
                Text { text: "Hello world!".into(), },
            )],
                                    events: BTreeMap::new(),
                                    namespace: None, }),
        ], });
        assert_eq!(div, expected)
    }
}

#[cfg(test)]
mod diff_tests_using_html_syntax {

    #![deny(warnings)]
    use super::*;
    use attributes::*;
    use events::*;
    use maplit::btreemap;
    use sauron_vdom::{diff,
                      Callback,
                      Event,
                      Patch,
                      Text,
                      Value};

    #[test]
    fn replace_node() {
        let old: Node<()> = div([], []);
        let new = span([], []);
        assert_eq!(diff(&old, &new),
                   vec![Patch::Replace(0, &span([], []))],
                   "Replace the root if the tag changed");

        let old: Node<()> = div([], [b([], [])]);
        let new = div([], [strong([], [])]);
        assert_eq!(diff(&old, &new),
                   vec![Patch::Replace(1, &strong([], []))],
                   "Replace a child node");

        let old: Node<()> = div([], [b([], [text("1")]), b([], [])]);
        let new = div([], [i([], [text("1")]), i([], [])]);
        assert_eq!(diff(&old, &new),
                   vec![Patch::Replace(1, &i([], [text("1")])),
                        Patch::Replace(3, &i([], [])),],
                   "Replace node with a child",)
    }

    #[test]
    fn add_children() {
        let old: Node<()> = div([], [b([], [])]); //{ <div> <b></b> </div> },
        let new = div([], [b([], []), html_element("new", [], [])]); //{ <div> <b></b> <new></new> </div> },
        assert_eq!(diff(&old, &new),
                   vec![Patch::AppendChildren(0,
                                              vec![&html_element("new",
                                                                 [],
                                                                 [])])],
                   "Added a new node to the root node",)
    }

    #[test]
    fn remove_nodes() {
        let old: Node<()> = div([], [b([], []), span([], [])]); //{ <div> <b></b> <span></span> </div> },
        let new = div([], []); //{ <div> </div> },

        assert_eq!(diff(&old, &new),
                   vec![Patch::TruncateChildren(0, 0)],
                   "Remove all child nodes at and after child sibling index 1",);

        let old: Node<()> = div([],
                                [span([],
                                      [b([], []),
                                       // This `i` tag will get removed
                                       i([], [])]),
                                 // This `strong` tag will get removed
                                 strong([], [])]);

        let new = div([], [span([], [b([], [])])]);

        assert_eq!(diff(&old, &new),
                   vec![Patch::TruncateChildren(0, 1),
                        Patch::TruncateChildren(1, 1)],
                   "Remove a child and a grandchild node",);

        let old: Node<()> = div([], [b([], [i([], []), i([], [])]), b([], [])]); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },
        let new = div([], [b([], [i([], [])]), i([], [])]); //{ <div> <b> <i></i> </b> <i></i> </div>},
        assert_eq!(diff(&old, &new),
                   vec![Patch::TruncateChildren(1, 1),
                        Patch::Replace(4, &i([], [])),],
                   "Removing child and change next node after parent",)
    }

    #[test]
    fn add_attributes() {
        let hello: Value = "hello".into();
        let attributes = btreemap! {
        "id" => &hello,
        };

        let old: Node<()> = div([], []); //{ <div> </div> },
        let new = div([id("hello")], []); //{ <div id="hello"> </div> },
        assert_eq!(diff(&old, &new),
                   vec![Patch::AddAttributes(0, attributes.clone())],
                   "Add attributes",);

        let old: Node<()> = div([id("foobar")], []); //{ <div id="foobar"> </div> },
        let new = div([id("hello")], []); //{ <div id="hello"> </div> },

        assert_eq!(diff(&old, &new),
                   vec![Patch::AddAttributes(0, attributes)],
                   "Change attribute",);
    }

    #[test]
    fn new_different_event_will_replace_what_was_first_set() {
        let func = |_| {
            println!("hello");
        };
        let hello: Callback<Event, ()> = func.into();
        let events = btreemap! {
        "click" => &hello,
        };

        let old = div([], []);
        let new = div([onclick(hello.clone())], []);
        assert_eq!(diff(&old, &new),
                   vec![Patch::AddEventListener(0, events.clone())],
                   "Add event listener",);

        let hello2: Callback<Event, ()> = func.into(); //recreated from the func closure, it will not be equal to the callback since the Rc points to a different address.
        assert_ne!(hello, hello2, "Same function, different Rc::new()");
        let old = div([onclick(hello.clone())], []);
        let new = div([onclick(hello2.clone())], []);

        assert_eq!(
            diff(&old, &new),
            vec![],
            "Even though a new callback is recated from the same closure
            It will point to a different Rc, which are not equal.
            However, since comparing the wrapped Fn is just not possible
            The diffing algorithmn will just leave what was first set as the event listener
            ",
        );
    }

    #[test]
    fn remove_attributes() {
        let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
        let new = div([], []); //{ <div> </div> },
        assert_eq!(diff(&old, &new),
                   vec![Patch::RemoveAttributes(0, vec!["id"])],
                   "Remove attributes",);
    }

    #[test]
    fn remove_events() {
        let old: Node<()> = div([onclick(|_| println!("hi"))], []);
        let new = div([], []);
        assert_eq!(diff(&old, &new),
                   vec![Patch::RemoveEventListener(0, vec!["click"])],
                   "Remove events",);
    }

    #[test]
    fn change_attribute() {
        let changed: Value = "changed".into();
        let attributes = btreemap! {
        "id" => &changed,
        };

        let old: Node<()> = div([id("hey-there")], []); //{ <div id="hey-there"></div> },
        let new = div([id("changed")], []); //{ <div id="changed"> </div> },

        assert_eq!(diff(&old, &new),
                   vec![Patch::AddAttributes(0, attributes)],
                   "Add attributes",);
    }

    #[test]
    fn replace_text_node() {
        let old: Node<()> = text("Old"); //{ Old },
        let new = text("New"); //{ New },

        assert_eq!(diff(&old, &new),
                   vec![Patch::ChangeText(0, &Text::new("New"))],
                   "Replace text node",);
    }

    // Initially motivated by having two elements where all that changed was an event listener
    // because right now we don't patch event listeners. So.. until we have a solution
    // for that we can just give them different keys to force a replace.
    #[test]
    fn replace_if_different_keys() {
        let old: Node<()> = div([key(1)], []); //{ <div key="1"> </div> },
        let new = div([key(2)], []); //{ <div key="2"> </div> },
        assert_eq!(
            diff(&old, &new),
            vec![Patch::Replace(0, &div([key(2)], []))],
            "If two nodes have different keys always generate a full replace.",
        );
    }
}
*/
