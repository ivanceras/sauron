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
//! facilitates parsing html string into sauron Node

pub use parser::{parse, parse_simple, tag_namespace, ParseError};

pub mod parser;
