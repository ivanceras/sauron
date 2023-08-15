use sauron::*;

#[test]
fn test_jss() {
    let css = jss!(
        ".layer": {
            background_color: "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = "\
        .layer {\
        \n  background-color: red;\
        \n  border: 1px solid green;\
        \n}\
        \n\
        \n.hide .layer {\
        \n  opacity: 0;\
        \n}\
        \n";
    assert_eq!(expected, css);
}

#[test]
fn test_jss_using_ident() {
    let css = jss!(
        ".layer": {
            background_color: "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = ".layer {\n  background-color: red;\n  border: 1px solid green;\n}\n\n.hide .layer {\n  opacity: 0;\n}\n";
    assert_eq!(expected, css);
}

#[test]
fn test_jss_ns() {
    let css = jss!(
        ".frame": {
            display: "block",
        },

        ".layer": {
            background_color: "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = ".frame {\n  display: block;\n}\n\n.layer {\n  background-color: red;\n  border: 1px solid green;\n}\n\n.hide .layer {\n  opacity: 0;\n}\n";
    assert_eq!(expected, css);
}
#[test]
fn test_jss_with_quoted_property_name() {
    let css = jss!(
        ".layer": {
            "background-color": "red",
            "border": "1px solid green",
        },

        ".hide .layer": {
            "opacity": 0,
        },
    );

    let expected = ".layer {\n  background-color: red;\n  border: 1px solid green;\n}\n\n.hide .layer {\n  opacity: 0;\n}\n";
    assert_eq!(expected, css);
}

#[test]
fn test_jss_with_mixed_quoting() {
    let css = jss!(
        ".block": {
            display: "block",
        },

        ".layer": {
            "background-color": "red",
            "user-select": "none",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );
    let expected = ".block {\n  display: block;\n}\n\n.layer {\n  background-color: red;\n  user-select: none;\n  border: 1px solid green;\n}\n\n.hide .layer {\n  opacity: 0;\n}\n";
    assert_eq!(expected, css);
}

#[test]
fn test_jss_ns_with_media_query() {
    let css = jss_with_media!(
        "@media screen and (max-width: 800px)": {
          ".layer": {
            width: "100%",
          }
        },
    );

    let expected = "\
        @media screen and (max-width: 800px) {\
            \n.layer {\
            \n  width: 100%;\
            \n}\
            \n\
            \n}\
            \n";
    assert_eq!(expected, css);
}
