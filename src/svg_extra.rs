//! These are valid svg tags and attributes, but not very commonly used.
//! These are put in separate module package to avoid conflicting imports of the most commonly used
//! tags/attributes

use crate::svg::SVG_NAMESPACE;

declare_svg_tags! {
    style;
    text;
    a;
    title;
}

pub mod attributes {
    use sauron_vdom::{
        builder::attr,
        Value,
    };
    declare_attributes! {
        style;
        width;
        height;
        id;
        path;
    }

    declare_attributes! {
        font_size => "font-size";
        font_family => "font-family";
    }

}
