//! Provides list of HTML and SVG tags, style properties
use crate::{
    html::{
        attributes::{HTML_ATTRS, HTML_ATTRS_SPECIAL},
        tags::{
            commons::HTML_TAGS, self_closing::HTML_SC_TAGS, HTML_TAGS_NON_COMMON,
            HTML_TAGS_WITH_MACRO_NON_COMMON,
        },
    },
    svg::{
        attributes::{SVG_ATTRS, SVG_ATTRS_SPECIAL, SVG_ATTRS_XLINK},
        tags::{commons::SVG_TAGS, special::SVG_TAGS_SPECIAL, SVG_TAGS_NON_COMMON},
        SVG_NAMESPACE,
    },
};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
pub use style_lookup::match_property;
mod style_lookup;

/// All of the svg tags
static ALL_SVG_TAGS: Lazy<BTreeSet<&&'static str>> = Lazy::new(|| {
    SVG_TAGS
        .iter()
        .chain(SVG_TAGS_NON_COMMON.iter())
        .chain(SVG_TAGS_SPECIAL.iter().map(|(_func, t)| t))
        .collect()
});

/// All of the html tags, excluding the SVG tags.
static ALL_HTML_TAGS: Lazy<BTreeSet<&&'static str>> = Lazy::new(|| {
    HTML_TAGS
        .iter()
        .chain(HTML_SC_TAGS.iter())
        .chain(HTML_TAGS_NON_COMMON.iter())
        .chain(HTML_TAGS_WITH_MACRO_NON_COMMON.iter())
        .collect()
});

static SELF_CLOSING_TAGS: Lazy<BTreeSet<&&'static str>> =
    Lazy::new(|| HTML_SC_TAGS.iter().collect());

static ALL_ATTRS: Lazy<BTreeMap<&'static str, &'static str>> = Lazy::new(|| {
    BTreeMap::from_iter(
        HTML_ATTRS
            .iter()
            .chain(SVG_ATTRS.iter())
            .map(|att| (*att, *att))
            .chain(
                HTML_ATTRS_SPECIAL
                    .iter()
                    .chain(SVG_ATTRS_SPECIAL.iter())
                    .chain(SVG_ATTRS_XLINK.iter())
                    .map(|(func, att)| (*func, *att)),
            ),
    )
});

/// Find the namespace of this tag
/// if the arg tag is an SVG tag, return the svg namespace
/// html tags don't need to have namespace while svg does, otherwise it will not be properly
/// mounted into the DOM
///
/// Limitations: `script`, `style`,and `a` used inside svg will return `None`, as these are also valid html tags.
pub fn tag_namespace(tag: &str) -> Option<&'static str> {
    let is_html = ALL_HTML_TAGS.contains(&tag);
    let is_svg = ALL_SVG_TAGS.contains(&tag);
    if !is_html {
        if is_svg {
            // we return the svg namespace only when the tag is not an html, but an svg tag
            // False negatives:
            // This means that script, style, a and title used inside in svg tag will not work
            // properly, since this 3 tags are valid html tags
            Some(SVG_NAMESPACE)
        } else {
            None
        }
    } else {
        None
    }
}

/// return the matching attribute
pub fn match_attribute(att: &str) -> Option<&'static str> {
    ALL_ATTRS
        .iter()
        .find(|(_k, v)| v == &&att)
        .map(|(_k, v)| *v)
}

/// given the attribute return the function name
pub fn attribute_function(att: &str) -> Option<&'static str> {
    ALL_ATTRS
        .iter()
        .find(|(_k, v)| v == &&att)
        .map(|(k, _v)| *k)
}

/// return the matching tag
pub fn match_tag(tag: &str) -> Option<&'static str> {
    ALL_HTML_TAGS
        .iter()
        .chain(ALL_SVG_TAGS.iter())
        .find(|t| **t == &tag)
        .map(|t| **t)
}

/// Returns true if this html tag is self closing
#[inline]
pub fn is_self_closing(tag: &str) -> bool {
    SELF_CLOSING_TAGS.contains(&tag)
}
