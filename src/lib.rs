//!
//!  **Sauron** is an web framework for creating fast and interactive client side web application,
//!  as well as server-side rendering for back-end web applications.
//!
//!
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png"
)]
#![deny(clippy::all)]
#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]

#[doc(inline)]
pub use prelude::*;

/// prelude
pub mod prelude {
    pub use sauron_core::prelude::*;
    pub use sauron_core::*;
    #[cfg(feature = "custom_element")]
    pub use sauron_macro::custom_element;
    #[cfg(feature = "with-node-macro")]
    pub use sauron_macro::node;
    #[cfg(feature = "with-jss")]
    pub use sauron_macro::{jss, jss_with_media, style};
    #[cfg(feature = "skip_diff")]
    pub use sauron_macro::skip_diff;
}
