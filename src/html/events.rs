//! https://developer.mozilla.org/en-US/docs/Web/Events

use crate::{
    prelude::Callback,
    Attribute,
    AttributeValue,
    Event,
};

/// on click event
pub fn on_click<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(Event) -> MSG + 'static,
{
    mt_dom::attr("click", AttributeValue::from_callback(Callback::from(f)))
}
