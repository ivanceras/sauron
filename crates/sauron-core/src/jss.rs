//! json css
//!
//!
use crate::html::attributes;

fn make_indent(n: usize, use_indents: bool) -> String {
    if use_indents {
        "    ".repeat(n)
    } else {
        String::from("")
    }
}
fn process_css_map(
    indent: usize,
    namespace: Option<&str>,
    css_map: &serde_json::Map<String, serde_json::Value>,
    use_indents: bool,
) -> String {
    let mut buffer = String::new();
    for (i, (classes, style_properties)) in css_map.iter().enumerate() {
        if i > 0 && use_indents {
            buffer += "\n";
        }
        if let Some(namespace) = &namespace {
            buffer += &format!(
                "{}{}",
                make_indent(indent, use_indents),
                selector_namespaced(namespace.to_string(), classes)
            );
        } else {
            buffer +=
                &format!("{}{}", make_indent(indent, use_indents), classes);
        }
        if use_indents {
            buffer += " {\n";
        }
        if let Some(style_properties) = style_properties.as_object() {
            for (prop, value) in style_properties {
                if value.is_object() {
                    buffer += &process_css_map(
                        indent + 1,
                        namespace,
                        style_properties,
                        use_indents,
                    );
                    if use_indents {
                        buffer += "\n";
                    }
                } else {
                    let value_str = match value {
                        serde_json::Value::String(s) => s.to_string(),
                        serde_json::Value::Number(v) => v.to_string(),
                        serde_json::Value::Bool(v) => v.to_string(),
                        _ => {
                            panic!(
                            "supported values are String, Number or Bool only"
                        )
                        }
                    };
                    buffer += &format!(
                        "{}{}: {};",
                        make_indent(indent + 1, use_indents),
                        prop,
                        value_str
                    );
                    if use_indents {
                        buffer += "\n";
                    }
                }
            }
        }
        buffer += &make_indent(indent, use_indents);
        buffer += "}";
    }
    buffer
}

/// process json to css transforming the selector
/// if class name is specified
pub fn process_css(
    namespace: Option<&str>,
    json: &serde_json::Value,
    use_indents: bool,
) -> String {
    let mut buffer = String::new();
    if let Some(css) = json.as_object() {
        buffer += &process_css_map(0, namespace, &css, use_indents);
    }
    buffer
}

/// jss with namespace
#[macro_export]
macro_rules! jss_ns {
    ($namespace: tt, $($tokens:tt)+) => {
        {
            let json = $crate::serde_json::json!($($tokens)*);
            $crate::jss::process_css(Some($namespace), &json, false)
        }
    };
}

/// jss macro
#[macro_export]
macro_rules! jss {
    ($($tokens:tt)+) => {
        {
            let json = $crate::serde_json::json!($($tokens)*);
            $crate::jss::process_css(None, &json, false)
        }
    };

}

/// TODO: use a real css-parser for this
/// prepend a namespace to the selector classes,
/// It does not affect element selector
/// example:
/// ```rust
/// use sauron_core::jss::selector_namespaced;
///
/// assert_eq!(".frame__text-anim", selector_namespaced("frame", ".text-anim"));
///
/// assert_eq!(
///     ".frame__hide .frame__corner",
///     selector_namespaced("frame", ".hide .corner")
/// );
///
/// assert_eq!(".frame__hide button", selector_namespaced("frame", ".hide button"));
/// assert_eq!(".frame__expand_corners,.frame__hovered", selector_namespaced("frame", ".expand_corners,.hovered"));
/// assert_eq!(".frame__expand_corners,.frame__hovered button .frame__highlight", selector_namespaced("frame", ".expand_corners,.hovered button .highlight"));
/// assert_eq!(".frame__expand_corners.frame__hovered button .frame__highlight", selector_namespaced("frame", ".expand_corners.hovered button .highlight"));
/// ```
pub fn selector_namespaced(
    namespace: impl ToString,
    selector_classes: impl ToString,
) -> String {
    let namespace = namespace.to_string();
    let selector_classes = selector_classes.to_string();
    let selector_trimmed = selector_classes.trim();

    if selector_trimmed == "." {
        format!(".{}", namespace)
    } else {
        selector_trimmed
            .split(" ")
            .map(|part| {
                let part = part.trim();
                if part.starts_with(".") {
                    let class_name = part.trim_start_matches(".");
                    class_name
                        .split(",")
                        .map(|cs_class| {
                            let cs_class = cs_class.trim_start_matches(".");
                            cs_class
                                .split(".")
                                .map(|dot_class| {
                                    format!(".{}__{}", namespace, dot_class)
                                })
                                .collect::<Vec<_>>()
                                .join("")
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                } else {
                    format!("{}", part)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn class_namespaced_str(
    namespace: impl ToString,
    class_names: impl ToString,
) -> String {
    let namespace = namespace.to_string();
    let class_names = class_names.to_string();
    let class_trimmed = class_names.trim();

    if class_trimmed.is_empty() {
        namespace
    } else {
        class_trimmed
            .split(" ")
            .map(|part| format!("{}__{}", namespace, part.trim()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// return a class attribute where the classnames are transformed with
/// namespace
/// example:
/// ```rust
/// use sauron_core::jss::class_namespaced;
/// use sauron_core::Attribute;
/// use sauron_core::html::attributes::class;
///
/// let expected: Attribute<()> = class("frame__border".to_string());
/// assert_eq!(expected, class_namespaced("frame", "border"));
///
/// let expected: Attribute<()> =
///     class("frame__border frame__corner".to_string());
/// assert_eq!(expected, class_namespaced("frame", "border corner"));
/// ```
pub fn class_namespaced<MSG>(
    namespace: impl ToString,
    class_names: impl ToString,
) -> crate::Attribute<MSG> {
    attributes::class(class_namespaced_str(namespace, class_names))
}

/// return a class namespaced with flag
pub fn classes_namespaced_flag<P, S, MSG>(
    namespace: impl ToString,
    pair: P,
) -> crate::Attribute<MSG>
where
    P: AsRef<[(S, bool)]>,
    S: ToString,
{
    let mut transformed = vec![];
    for (class_name, flag) in pair.as_ref() {
        transformed.push((
            class_namespaced_str(namespace.to_string(), class_name.to_string()),
            *flag,
        ));
    }
    attributes::classes_flag(transformed)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{html::attributes::class, Attribute};

    #[test]
    fn test_jss() {
        let css = jss!({
            ".layer": {
                "background-color": "red",
                "border": "1px solid green",
            },

            ".hide .layer": {
                "opacity": 0,
            },
        });

        let expected = r#".layer {
    background-color: red;
    border: 1px solid green;
}
.hide .layer {
    opacity: 0;
}"#;
        println!("{}", css);
        assert_eq!(expected, css);
    }

    #[test]
    fn test_jss_ns() {
        let css = jss_ns!("frame",{
            ".": {
                "display": "block",
            },

            ".layer": {
                "background-color": "red",
                "border": "1px solid green",
            },

            ".hide .layer": {
                "opacity": 0,
            },
        });

        let expected = r#".frame {
    display: block;
}
.frame__layer {
    background-color: red;
    border: 1px solid green;
}
.frame__hide .frame__layer {
    opacity: 0;
}"#;
        println!("{}", css);
        assert_eq!(expected, css);
    }

    #[test]
    fn test_jss_ns_with_media_query() {
        let css = jss_ns!("frame",{
            ".": {
                "display": "block",
            },

            ".layer": {
                "background-color": "red",
                "border": "1px solid green",
            },

            "@media screen and (max-width: 800px)": {
              ".layer": {
                "width": "100%",
              }
            },

            ".hide .layer": {
                "opacity": 0,
            },
        });

        let expected = r#".frame {
    display: block;
}
.frame__layer {
    background-color: red;
    border: 1px solid green;
}
@media screen and (max-width: 800px) {
    .frame__layer {
        width: 100%;
    }
}
.frame__hide .frame__layer {
    opacity: 0;
}"#;
        println!("{}", css);
        assert_eq!(expected, css);
    }

    #[test]
    fn test_selector_ns() {
        assert_eq!(".frame", selector_namespaced("frame", "."));
        assert_eq!(
            ".frame__hide .frame__corner",
            selector_namespaced("frame", ".hide .corner")
        );
    }

    #[test]
    fn test_class_namespaced() {
        let expected: Attribute<()> = class("frame__border".to_string());
        assert_eq!(expected, class_namespaced("frame", "border"));

        let expected: Attribute<()> =
            class("frame__border frame__corner".to_string());
        assert_eq!(expected, class_namespaced("frame", "border corner"));
    }
}
