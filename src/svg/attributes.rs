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

    stroke_width => "stroke-width";

    stop_color => "stop-color";

    stop_opacity => "stop-opacity";

    stroke_linecap => "stroke-linecap";

    stroke_dasharray => "stroke-dasharray";

    stroke_dashoffset => "stroke-dashoffset";

    transform_origin => "transform-origin";

    stroke_opacity => "stroke-opacity";
}

//TODO: add the rest of the attributes that are used in svg elements
