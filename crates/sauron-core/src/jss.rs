//! json css
//!
//!
use crate::html::attributes;

/// process json to css transforming the selector
/// if class name is specified
pub fn process_css(namespace: Option<&str>, json: serde_json::Value) -> String {
    let mut buffer = String::new();
    if let Some(css) = json.as_object() {
        for (index, (classes, style_properties)) in css.iter().enumerate() {
            if index > 0 {
                buffer += "\n";
            }
            if let Some(namespace) = &namespace {
                buffer += &selector_namespaced(namespace.to_string(), classes);
            } else {
                buffer += classes;
            }
            buffer += " {\n";
            if let Some(style_properties) = style_properties.as_object() {
                for (prop, value) in style_properties {
                    let value_str = match value {
                        serde_json::Value::String(s) => s.to_string(),
                        serde_json::Value::Number(v) => v.to_string(),
                        serde_json::Value::Bool(v) => v.to_string(),
                        _ => panic!(
                            "supported values are String, Number or Bool only"
                        ),
                    };
                    buffer += &format!("    {}: {};\n", prop, value_str);
                }
            }
            buffer += "}";
        }
    }
    buffer
}

/// jss with namespace
#[macro_export]
macro_rules! jss_ns {
    ($namespace: tt, $($tokens:tt)+) => {
        {
            let json = serde_json::json!($($tokens)*);
            $crate::jss::process_css(Some($namespace), json)
        }
    };
}

/// jss macro
#[macro_export]
macro_rules! jss {
    ($($tokens:tt)+) => {
        {
            let json = serde_json::json!($($tokens)*);
            $crate::jss::process_css(None, json)
        }
    };

}

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
                    format!(".{}__{}", namespace, part.trim_start_matches("."))
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
    use crate::html::attributes::class;
    use crate::Attribute;

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
