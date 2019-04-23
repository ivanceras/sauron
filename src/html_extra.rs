//! These are valid html tags and attributes, but not very commonly used.
//! These are put in separate module package to avoid conflicting imports of the most commonly used
//! tags/attributes
builder_constructors! {
    /// Build a
    /// [`<style>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
    /// element.
    style;
}
pub mod attributes {
    use sauron_vdom::builder::attr;
    use sauron_vdom::builder::Attribute;
    use sauron_vdom::Value;
    declare_attributes! {
        span;
        label;
    }
}
