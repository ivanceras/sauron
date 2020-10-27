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

pub use parser::{match_attribute_function, parse, parse_simple, ParseError};

mod parser;
