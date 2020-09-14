#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//! This is the core of sauron
//!
#[macro_use]
extern crate doc_comment;

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "with-dom")] {
    pub mod dom;
    pub use dom::*;
    /// Map the Event to DomEvent, which are browser events
    pub type Event = web_sys::Event;
    pub use web_sys;
    pub use wasm_bindgen;
}}

// reexport serde_json
pub use serde_json;

/// When event is not needed, such as just rendering the dom
/// tree in server side application
#[cfg(not(feature = "with-dom"))]
pub type Event = ();

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
#[macro_use]
pub mod jss;
mod render;

pub use render::Render;

use html::attributes::AttributeValue;
use mt_dom::diff_with_key;

pub use mt_dom;

/// Prelude simplifies the imports from sauron
/// This imports the necessary functions to build
/// a basic sauron app.
pub mod prelude {
    pub use crate::{
        html::{
            attributes::{
                attr,
                *,
            },
            tags::{
                commons::*,
                *,
            },
            units::*,
            *,
        },
        svg::{
            attributes::*,
            tags::commons::*,
            *,
        },
        *,
    };
    pub use serde_json;
    #[cfg(feature = "with-dom")]
    pub use wasm_bindgen::prelude::*;
    #[cfg(feature = "with-dom")]
    pub use web_sys;
}

/// namespace type in node, which could be change to an enum
pub type Namespace = &'static str;
/// tags are using static str for now, can also be enum tags
pub type Tag = &'static str;
/// attribute keys
pub type AttributeKey = &'static str;

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> =
    mt_dom::Node<Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Element type with tag and attribute name type set to &'static str
pub type Element<MSG> =
    mt_dom::Element<Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Patch as result of diffing the current_vdom and the new vdom.
/// The tag and attribute name types is set to &'static str
pub type Patch<'a, MSG> =
    mt_dom::Patch<'a, Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Attribute type used in sauron where the type of the Attribute name is &'static str
pub type Attribute<MSG> =
    mt_dom::Attribute<Namespace, AttributeKey, AttributeValue, Event, MSG>;

/// Callback where Event type is supplied
pub type Callback<MSG> = mt_dom::Callback<Event, MSG>;

/// This is a sauron html specific functionality
/// diff 2 nodes with attribute using `&'static str` instead of generic ATT
pub fn diff<'a, MSG>(
    old: &'a Node<MSG>,
    new: &'a Node<MSG>,
) -> Vec<Patch<'a, MSG>>
where
    MSG: 'static,
{
    diff_with_key(old, new, &"key")
}
