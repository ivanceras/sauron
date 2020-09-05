//! json css
//!
//!

/// jss macro
#[macro_export]
macro_rules! jss {
    ($($tokens:tt)+) => {
        {
            let json = serde_json::json!($($tokens)*);
            let mut buffer = String::new();
            if let Some(css) = json.as_object(){
                for (index,(classes, style_properties)) in css.iter().enumerate(){
                    if index > 0 {
                        buffer += "\n";
                    }
                    buffer += &classes;
                    buffer += " {\n";
                    if let Some(style_properties) = style_properties.as_object(){
                        for (prop, value) in style_properties{
                            let value_str = match value{
                                serde_json::Value::String(s) => s.to_string(),
                                serde_json::Value::Number(v) => v.to_string(),
                                serde_json::Value::Bool(v) => v.to_string(),
                                _ => panic!("supported values are String, Number or Bool only"),
                            };
                            buffer += &format!("    {}: {};\n", prop, value_str);
                        }
                    }
                    buffer += "}";
                }
            }
            buffer
        }
    }
}

/// prepend a namespace to the selector classeses
/// example:
/// ```rust
/// use sauron_core::jss::selector_namespaced;
/// assert_eq!(".frame__text-anim", selector_namespaced("frame", "text-anim"))
/// ```
pub fn selector_namespaced(
    namespace: impl ToString,
    selector_classes: impl ToString,
) -> String {
    let namespace = namespace.to_string();
    let selector_classes = selector_classes.to_string();
    let selector_trimmed = selector_classes.trim();

    if selector_trimmed.is_empty() {
        format!(".{}", namespace)
    } else {
        selector_trimmed
            .split(" ")
            .map(|part| format!(".{}__{}", namespace, part.trim()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// return a class attribute where the classnames are transformed with
/// namespace
pub fn class_namespaced<MSG>(
    namespace: impl ToString,
    class_names: impl ToString,
) -> crate::Attribute<MSG> {
    let namespace = namespace.to_string();
    let class_names = class_names.to_string();
    let class_trimmed = class_names.trim();

    if class_trimmed.is_empty() {
        crate::html::attributes::class(namespace)
    } else {
        crate::html::attributes::class(
            class_trimmed
                .split(" ")
                .map(|part| format!("{}__{}", namespace, part.trim()))
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
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
    fn test_selector_ns() {
        assert_eq!(".frame", selector_namespaced("frame", ""));
        assert_eq!(
            ".frame__hide .frame__corner",
            selector_namespaced("frame", "hide corner")
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
