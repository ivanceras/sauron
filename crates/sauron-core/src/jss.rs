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
                                _ => "".to_string(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("background-color", backgroundColor);
    }

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
}
