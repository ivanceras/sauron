#![deny(
    warnings,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//! a library to parse markdown and convert it into sauron virtual node
pub use markdown::{markdown, render_markdown};
mod markdown;
