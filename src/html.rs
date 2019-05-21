use std::convert::AsRef;

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

#[inline]
pub fn html_element<A, C, MSG>(
    tag: &'static str,
    attrs: A,
    children: C,
) -> Node<MSG>
where
    C: AsRef<[Node<MSG>]>,
    A: AsRef<[Attribute<MSG>]>,
    MSG: Clone,
{
    sauron_vdom::builder::element(tag, attrs, children)
}

#[inline]
pub fn html_element_ns<A, C, MSG>(
    tag: &'static str,
    namespace: &'static str,
    attrs: A,
    children: C,
) -> Node<MSG>
where
    C: AsRef<[Node<MSG>]>,
    A: AsRef<[Attribute<MSG>]>,
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
    /// [`<template>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
    /// element.
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
