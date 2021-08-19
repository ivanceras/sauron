use once_cell::sync::Lazy;
use sauron_core::{
    html::tags::{
        HTML_SC_TAGS, HTML_TAGS, HTML_TAGS_NON_COMMON,
        HTML_TAGS_WITH_MACRO_NON_COMMON,
    },
    svg::{
        tags::{SVG_TAGS, SVG_TAGS_NON_COMMON, SVG_TAGS_SPECIAL},
        SVG_NAMESPACE,
    },
};
use std::collections::HashSet;
use std::iter::FromIterator;

/// All of the svg tags
static ALL_SVG_TAGS: Lazy<HashSet<&&'static str>> = Lazy::new(|| {
    HashSet::from_iter(
        SVG_TAGS
            .iter()
            .chain(SVG_TAGS_NON_COMMON.iter())
            .chain(SVG_TAGS_SPECIAL.iter().map(|(_func, t)| t)),
    )
});

/// All of the html tags, excluding the SVG tags.
static ALL_HTML_TAGS: Lazy<HashSet<&&'static str>> = Lazy::new(|| {
    HashSet::from_iter(
        HTML_TAGS
            .iter()
            .chain(HTML_SC_TAGS.iter())
            .chain(HTML_TAGS_NON_COMMON.iter())
            .chain(HTML_TAGS_WITH_MACRO_NON_COMMON.iter()),
    )
});

static SELF_CLOSING_TAGS: Lazy<HashSet<&&'static str>> =
    Lazy::new(|| HashSet::from_iter(HTML_SC_TAGS.iter()));

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

/// Returns true if this html tag is self closing
#[inline]
pub fn is_self_closing(tag: &str) -> bool {
    SELF_CLOSING_TAGS.contains(&tag)
}
