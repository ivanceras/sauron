pub use sauron_vdom::builder::attr;
use sauron_vdom::builder::Attribute;
use sauron_vdom::Value;

// svg attributes
declare_attributes! {
    x;
    y;
    cx;
    cy;
    r;
    x1;
    y1;
    x2;
    y2;
    xmlns;
    offset;
    stroke;
    fill;
    transform;
    transition;
}
// attributes that has dash
declare_attributes! {

    #[allow(non_snake_case)]
    strokeWidth => "stroke-width";

    #[allow(non_snake_case)]
    stopColor => "stop-color";

    #[allow(non_snake_case)]
    stopOpacity => "stop-opacity";

    #[allow(non_snake_case)]
    strokeLinecap => "stroke-linecap";

    #[allow(non_snake_case)]
    strokeDasharray => "stroke-dasharray";

    #[allow(non_snake_case)]
    strokeDashoffset => "stroke-dashoffset";

    #[allow(non_snake_case)]
    transformOrigin => "transform-origin";

    #[allow(non_snake_case)]
    strokeOpacity => "stroke-opacity";
}

//TODO: add the rest of the attributes that are used in svg elements
