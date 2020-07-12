#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//! A utility crate which provides conversion of html text into sauron view syntax
//!
use sauron_parse::ParseError;
pub use to_syntax::ToSyntax;

mod to_syntax;

/// converts html to sauron view syntax
pub fn html_to_syntax(
    html: &str,
    use_macro: bool,
) -> Result<String, ParseError> {
    log::info!("input: {}", html);
    println!("input: {}", html);
    match sauron_parse::parse::<()>(html) {
        Ok(root_node) => {
            let mut buffer = String::new();
            if let Some(root_node) = root_node {
                root_node.to_syntax(&mut buffer, use_macro, 0)?;
            }
            Ok(buffer)
        }
        Err(e) => {
            log::error!("error: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simpe_convert() {
        let input = r#"
        <div>content1</div>
        <div>content2</div>
        <div>content3</div>
            "#;

        let expected = r#"html!([],[
    div!([],[text("content1")]),
    div!([],[text("content2")]),
    div!([],[text("content3")]),
])"#;
        let syntax = html_to_syntax(input, true).expect("must not fail");
        println!("syntax: {}", syntax);
        assert_eq!(expected, syntax);
    }

    #[test]
    fn simple_html_parse() {
        let input = r#"<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Interactive sauron app</title>
    <style type="text/css">
        body {
            font-family: "Fira Sans", "Courier New";
        }
    </style>
</head>
<body style='margin: 0; padding: 0; width: 100%; height: 100%;'>
  <div id="web-app" style='width: 100%; height: 100%;'>
      #HTML_INSERTED_HERE_BY_SERVER#
  </div>
  <!-- This is a comment -->
</body>
</html>"#;
        let expected = r#"html!([lang("en"),],[
    head!([],[
        meta!([charset("UTF-8"),],[]),
        meta!([content("width=device-width, initial-scale=1"),name("viewport"),],[]),
        title!([],[text("Interactive sauron app")]),
        style!([r#type("text/css"),],[text("
        body {
            font-family: "Fira Sans", "Courier New";
        }
    ")]),
    ]),
    body!([style("margin: 0; padding: 0; width: 100%; height: 100%;"),],[
        div!([id("web-app"),style("width: 100%; height: 100%;"),],[text("
      #HTML_INSERTED_HERE_BY_SERVER#
  ")]),
    ]),
])"#;
        let syntax = html_to_syntax(input, true).expect("must not fail");
        println!("syntax: {}", syntax);
        assert_eq!(expected, syntax);
    }

    #[test]
    fn simple_svg_parse() {
        let input = r#"
<svg height="400" viewBox="0 0 600 400" width="600" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <defs>
        <filter id="shadow">
            <feDropShadow dx="2" dy="1" stdDeviation="0.2"></feDropShadow>
        </filter>
    </defs>
    <image height="400" xlink:href="data:image/jpeg;base64,/9j/4AAQSkZJRgABA" width="600" x="0" y="0"></image>
    <text fill="red" font-family="monospace" font-size="40" stroke="white" stroke-width="1" style="filter:url(#shadow);" x="65" y="55">John Smith</text>
    <text fill="white" font-family="monospace" font-size="20" style="filter:url(#shadow);" x="100" y="100">10101011</text>
    <text fill="red" font-family="monospace" font-size="50" style="filter:url(#shadow);" width="500" x="20" y="200">Happy birthday</text>
</svg>
"#;
        let expected = r#"html!([],[
    svg!([height(400),viewBox("0 0 600 400"),width(600),xmlns("http://www.w3.org/2000/svg"),],[
        defs!([],[
            filter!([id("shadow"),],[
                feDropShadow!([dx(2),dy(1),stdDeviation(0.2),],[]),
            ]),
        ]),
        image!([height(400),href("data:image/jpeg;base64,/9j/4AAQSkZJRgABA"),width(600),x(0),y(0),],[]),
        text!([fill("red"),font_family("monospace"),font_size(40),stroke("white"),stroke_width(1),style("filter:url(#shadow);"),x(65),y(55),],[text("John Smith")]),
        text!([fill("white"),font_family("monospace"),font_size(20),style("filter:url(#shadow);"),x(100),y(100),],[text("10101011")]),
        text!([fill("red"),font_family("monospace"),font_size(50),style("filter:url(#shadow);"),width(500),x(20),y(200),],[text("Happy birthday")]),
    ]),
])"#;
        let syntax = html_to_syntax(input, true).expect("must not fail");
        println!("syntax: {}", syntax);
        assert_eq!(expected, syntax);
    }
}
