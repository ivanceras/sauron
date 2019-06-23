//! These are valid html tags and attributes, but not very commonly used.
//! These are put in separate module package to avoid conflicting imports of the most commonly used
//! tags/attributes
declare_tags! {
    style;
    html;
    title;
    slot;
}
pub mod attributes {
    use sauron_vdom::{
        builder::attr,
        Value,
    };

    declare_attributes! {
        span;
        label;
        form;
        code;
    }
}
