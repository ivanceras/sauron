//! provides functionalities and macro for building html elements

macro_rules! declare_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment!{
                concat!("Creates an html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",stringify!($name),") element"),

            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG>(attrs: impl IntoIterator<Item = $crate::vdom::Attribute<MSG>>, children: impl IntoIterator<Item = $crate::vdom::Node<MSG>>) -> $crate::vdom::Node<MSG>
                {
                    $crate::html::html_element(None, stringify!($name), attrs, children, false)
                }
            }

         )*
    }
}

/// declare self closing tags
macro_rules! declare_sc_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {

        /// self closing tags
        pub(crate) mod self_closing{
            $(
                doc_comment!{
                    concat!("Creates an html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",stringify!($name),") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG>(attrs: impl IntoIterator<Item = $crate::vdom::Attribute<MSG>>, children: impl IntoIterator<Item = $crate::vdom::Node<MSG>>) -> $crate::vdom::Node<MSG>
                    {
                        $crate::html::html_element(None, stringify!($name), attrs, children, true)
                    }
                }

             )*
        }

        #[cfg(feature = "with-lookup")]
        /// These are the self closing tags such as `<input/>`, `<br/>`,
        pub const HTML_SC_TAGS: &[&'static str] = &[$(stringify!($name),)*];
    }
}

macro_rules! declare_common_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

         declare_tags! { $($name;)* }

        #[cfg(feature = "with-lookup")]
        /// These are the comonly used html tags such as div, input, buttons,.. etc
        pub const HTML_TAGS: &[&'static str] = &[$(stringify!($name),)*];
    };
}

macro_rules! declare_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        /// These are html tags which are non commonly used.
        /// Put together in this collection to avoid import conflicts with the commonly used
        /// ones.
        pub const HTML_TAGS_NON_COMMON: &[&'static str] = &[$(stringify!($name),)*];
    }
}

macro_rules! declare_tags_and_macro_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        /// These are html tags with macro which are non commonly used.
        /// Put together in this collection to avoid import conflicts with the commonly used
        /// ones.
        pub const HTML_TAGS_WITH_MACRO_NON_COMMON: &[&'static str] = &[$(stringify!($name),)*];
    }
}

/// commonly used html tags
pub mod commons {
    // Organized in the same order as
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Element
    //
    // Does not include obsolete elements.
    declare_common_tags_and_macro! {
        head;
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
        html;
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
        cite;
        code;
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
        audio;
        map;
        video;
        iframe;
        object;
        picture;
        canvas;
        noscript;
        script;
        del;
        ins;
        caption;
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
}

declare_tags_non_common! {
    style;  //  conflicts with html::attributes::style, attribute::style    > tags::style
}

// These are non-common tags
// which the users need to explicitly import using
// html::tags::style, html::tags::html, etc
//
declare_tags_and_macro_non_common! {
    title; // conflicts with html::attributes::title  , attributes::title   > tags::title
    slot;  // conflicts with html::attributes::slot   , attrributes::slot   > tags::slot
    data;  // data for local variable is commonly used everywhere
}

// self closing tags such as `<input/>, `<br/>`
declare_sc_tags! {
    area;
    base;
    br;
    col;
    command;
    embed;
    hr;
    img;
    input;
    keygen;
    link;
    meta;
    param;
    source;
    track;
    wbr;
}
