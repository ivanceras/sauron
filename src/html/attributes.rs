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

declare_attributes! {
    accesskey;

    autocapitalize;

    class;

    contextmenu;

    draggable;

    dropzone;

    hidden;

    id;

    inputmode;

    is;

    itemid;

    itemprop;

    itemref;

    itemscope;

    itemtype;

    lang;

    slot;

    spellcheck;

    style;

    tabindex;

    title;

    translate;


}

// special case for type attribute, since type is a rust keyword
declare_attributes! {
    r#type => "type";
}

// common attributes
declare_attributes! {
    value;
    key;
    placeholder;
    display;
    checked;
    target;
    href;
}

// sizing attribtes
declare_attributes! {
    width;
    height;
    rows;
    cols;
}

// attributes with dash
declare_attributes! {

    #[allow(non_snake_case)]
    fontFamily => "font-family";

    #[allow(non_snake_case)]
    fontSize => "font-size";

    #[allow(non_snake_case)]
    flexDirection => "flex-direction";
}
//TODO: add the rest of attributes from the html specs
