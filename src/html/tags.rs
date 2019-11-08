macro_rules! declare_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG>(attrs: Vec<$crate::Attribute<MSG>>, children: Vec<$crate::Node<MSG>>) -> $crate::Node<MSG>
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
pub(super) mod commons {
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
}

// These are non-common tags
// which the users need to explicitly import using
// html::tags::style, html::tags::html, etc
//
declare_tags! {
    style;
    html;
    title;
    slot;
}
