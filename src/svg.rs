pub use sauron_vdom::builder::{attr, element, element_ns};

pub mod attributes;

builder_constructors! {
    // SVG components

    /// Build a
    /// [`<svg>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg)
    /// element.
    svg <> "http://www.w3.org/2000/svg" ;
    /// Build a
    /// [`<path>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path)
    /// element.
    path <> "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<circle>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/circle)
    /// element.
    circle <>  "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<ellipse>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/ellipse)
    /// element.
    ellipse <> "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<line>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/line)
    /// element.
    line <> "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<polygon>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polygon)
    /// element.
    polygon <> "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<polyline>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polyline)
    /// element.
    polyline <> "http://www.w3.org/2000/svg";
    /// Build a
    /// [`<rect>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect)
    /// element.
    rect <> "http://www.w3.org/2000/svg";

    /// Build a
    /// [`<text>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text)
    /// element.
    text <> "http://www.w3.org/2000/svg";
}

builder_constructors! {
    defs <> "http://www.w3.org/2000/svg";
    stop <> "http://www.w3.org/2000/svg";
    g <> "http://www.w3.org/2000/svg";
    tspan <> "http://www.w3.org/2000/svg";
    #[allow(non_snake_case)]
    radialGradient <> "http://www.w3.org/2000/svg";
}
