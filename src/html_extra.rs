//! These are valid html tags and attributes, but not very commonly used.
//! These are put in separate module package to avoid conflicting imports of the most commonly used
//! tags/attributes
declare_tags! {
    /// Build a
    /// [`<style>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
    /// element.
    style;
    /// Build a
    /// [`<html>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)
    /// element.
    html;
    /// Build a
    /// [`<title>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    /// element.
    title;

    /// Build a
    /// [`<slot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
    /// element.
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
    }
}
