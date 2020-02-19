pub use sauron_vdom::builder::{
    attr,
    attr_ns,
};
use sauron_vdom::Value;

pub use crate::html::attributes::classes_flag;

pub(in crate) const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

macro_rules! declare_xlink_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<V, MSG>(v: V) -> crate::Attribute<MSG>
                where V: Into<Value>,
                {
                    attr_ns(Some(XLINK_NAMESPACE), $attribute, v)
                }
         )*
    }
}

// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
// complete list svg attributes
declare_attributes! {
    accumulate;
    additive;
    allowReorder;
    alphabetic;
    amplitude;
    ascent;
    attributeName;
    attributeType;
    autoReverse;
    azimuth;
    baseFrequency;
    baseProfile;
    bbox;
    begin;
    bias;
    by;
    calcMode;
    clip;
    clipPathUnits;
    color;
    contentScriptType;
    contentStyleType;
    cursor;
    cx;
    cy;
    d;
    decelerate;
    descent;
    diffuseConstant;
    direction;
    display;
    divisor;
    dur;
    dx;
    dy;
    edgeMode;
    elevation;
    end;
    exponent;
    externalResourcesRequired;
    fill;
    filter;
    filterRes;
    filterUnits;
    format;
    from;
    fr;
    fx;
    fy;
    g1;
    g2;
    glyphRef;
    gradientTransform;
    gradientUnits;
    hanging;
    hreflang;
    ideographic;
    in2;
    intercept;
    k;
    k1;
    k2;
    k3;
    k4;
    kernelMatrix;
    kernelUnitLength;
    kerning;
    keyPoints;
    keySplines;
    keyTimes;
    lang;
    lengthAdjust;
    limitingConeAngle;
    local;
    markerHeight;
    markerUnits;
    markerWidth;
    mask;
    maskContentUnits;
    maskUnits;
    mathematical;
    max;
    media;
    method;
    min;
    mode;
    name;
    numOctaves;
    offset;
    opacity;
    operator;
    order;
    orient;
    orientation;
    origin;
    overflow;
    pathLength;
    patternContentUnits;
    patternTransform;
    patternUnits;
    ping;
    points;
    pointsAtX;
    pointsAtY;
    pointsAtZ;
    preserveAlpha;
    preserveAspectRatio;
    primitiveUnits;
    r;
    radius;
    referrerPolicy;
    refX;
    refY;
    rel;
    repeatCount;
    repeatDur;
    requiredExtensions;
    requiredFeatures;
    restart;
    result;
    rotate;
    rx;
    ry;
    scale;
    seed;
    slope;
    spacing;
    specularConstant;
    specularExponent;
    speed;
    spreadMethod;
    startOffset;
    stdDeviation;
    stemh;
    stemv;
    stitchTiles;
    string;
    stroke;
    surfaceScale;
    systemLanguage;
    tabindex;
    tableValues;
    target;
    targetX;
    targetY;
    textLength;
    to;
    transform;
    u1;
    u2;
    unicode;
    values;
    version;
    viewBox;
    viewTarget;
    visibility;
    widths;
    x;
    x1;
    x2;
    xChannelSelector;
    xmlns;
    y;
    y1;
    y2;
    yChannelSelector;
    z;
    zoomAndPan;
}

// attributes that has dash
declare_attributes! {
    accent_height => "accent-height";
    alignment_baseline => "alignment-baseline";
    arabic_form => "arabic-form";
    baseline_shift => "baseline-shift";
    cap_height => "cap-height";
    clip_path => "clip-path";
    clip_rule => "clip-rule";
    color_interpolation => "color-interpolation";
    color_interpolation_filters => "color-interpolation-filters";
    color_profile => "color-profile";
    color_rendering => "color-rendering";
    dominant_baseline => "dominant-baseline";
    enable_background => "enable-background";
    fill_opacity => "fill-opacity";
    fill_rule => "fill-rule";
    flood_color => "flood-color";
    flood_opacity => "flood-opacity";
    font_size_adjust => "font-size-adjust";
    font_stretch => "font-stretch";
    font_style => "font-style";
    font_variant => "font-variant";
    font_weight => "font-weight";
    glyph_name => "glyph-name";
    glyph_orientation_horizontal => "glyph-orientation-horizontal";
    glyph_orientation_vertical => "glyph-orientation-vertical";
    horiz_adv_x => "horiz-adv-x";
    horiz_origin_x => "horiz-origin-x";
    image_rendering => "image-rendering";
    letter_spacing => "letter-spacing";
    lighting_color => "lighting-color";
    marker_end => "marker-end";
    marker_mid => "marker-mid";
    marker_start => "marker-start";
    overline_position => "overline-position";
    overline_thickness => "overline-thickness";
    panose_1 => "panose-1";
    paint_order => "paint-order";
    pointer_events => "pointer-events";
    rendering_intent => "rendering-intent";
    shape_rendering => "shape-rendering";
    stop_color => "stop-color";
    stop_opacity => "stop-opacity";
    strikethrough_position => "strikethrough-position";
    strikethrough_thickness => "strikethrough-thickness";
    stroke_dasharray => "stroke-dasharray";
    stroke_dashoffset => "stroke-dashoffset";
    stroke_linecap => "stroke-linecap";
    stroke_linejoin => "stroke-linejoin";
    stroke_miterlimit => "stroke-miterlimit";
    stroke_opacity => "stroke-opacity";
    stroke_width => "stroke-width";
    text_anchor => "text-anchor";
    text_decoration => "text-decoration";
    text_rendering => "text-rendering";
    underline_position => "underline-position";
    underline_thickness => "underline-thickness";
    unicode_bidi => "unicode-bidi";
    unicode_range => "unicode-range";
    units_per_em => "units-per-em";
    v_alphabetic => "v-alphabetic";
    v_hanging => "v-hanging";
    v_ideographic => "v-ideographic";
    v_mathematical => "v-mathematical";
    vector_effect => "vector-effect";
    vert_adv_y => "vert-adv-y";
    vert_origin_x => "vert-origin-x";
    vert_origin_y => "vert-origin-y";
    word_spacing => "word-spacing";
    writing_mode => "writing-mode";
    x_height => "x-height";
    xml_base => "xml:base";
    xml_lang => "xml:lang";
    xml_space => "xml:space";
}

declare_attributes! {
    r#in => "in";
    r#type => "type";
}

declare_xlink_attributes! {
    xlink_actuate => "actuate";
    xlink_arcrole => "arcrole";
    xlink_href => "href";
    xlink_role => "role";
    xlink_show => "show";
    xlink_title => "title";
    xlink_type => "type";
}

pub(crate) const SVG_ATTRS: [&'static str; 250] = [
    "accumulate",
    "additive",
    "allowReorder",
    "alphabetic",
    "amplitude",
    "ascent",
    "attributeName",
    "attributeType",
    "autoReverse",
    "azimuth",
    "baseFrequency",
    "baseProfile",
    "bbox",
    "begin",
    "bias",
    "by",
    "calcMode",
    "clip",
    "clipPathUnits",
    "color",
    "contentScriptType",
    "contentStyleType",
    "cursor",
    "cx",
    "cy",
    "d",
    "decelerate",
    "descent",
    "diffuseConstant",
    "direction",
    "display",
    "divisor",
    "dur",
    "dx",
    "dy",
    "edgeMode",
    "elevation",
    "end",
    "exponent",
    "externalResourcesRequired",
    "fill",
    "filter",
    "filterRes",
    "filterUnits",
    "format",
    "from",
    "fr",
    "fx",
    "fy",
    "g1",
    "g2",
    "glyphRef",
    "gradientTransform",
    "gradientUnits",
    "hanging",
    "hreflang",
    "ideographic",
    "in2",
    "intercept",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kernelMatrix",
    "kernelUnitLength",
    "kerning",
    "keyPoints",
    "keySplines",
    "keyTimes",
    "lang",
    "lengthAdjust",
    "limitingConeAngle",
    "local",
    "markerHeight",
    "markerUnits",
    "markerWidth",
    "mask",
    "maskContentUnits",
    "maskUnits",
    "mathematical",
    "max",
    "media",
    "method",
    "min",
    "mode",
    "name",
    "numOctaves",
    "offset",
    "opacity",
    "operator",
    "order",
    "orient",
    "orientation",
    "origin",
    "overflow",
    "pathLength",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "ping",
    "points",
    "pointsAtX",
    "pointsAtY",
    "pointsAtZ",
    "preserveAlpha",
    "preserveAspectRatio",
    "primitiveUnits",
    "r",
    "radius",
    "referrerPolicy",
    "refX",
    "refY",
    "rel",
    "repeatCount",
    "repeatDur",
    "requiredExtensions",
    "requiredFeatures",
    "restart",
    "result",
    "rotate",
    "rx",
    "ry",
    "scale",
    "seed",
    "slope",
    "spacing",
    "specularConstant",
    "specularExponent",
    "speed",
    "spreadMethod",
    "startOffset",
    "stdDeviation",
    "stemh",
    "stemv",
    "stitchTiles",
    "string",
    "stroke",
    "surfaceScale",
    "systemLanguage",
    "tabindex",
    "tableValues",
    "target",
    "targetX",
    "targetY",
    "textLength",
    "to",
    "transform",
    "u1",
    "u2",
    "unicode",
    "values",
    "version",
    "viewBox",
    "viewTarget",
    "visibility",
    "widths",
    "x",
    "x1",
    "x2",
    "xChannelSelector",
    "xmlns",
    "y",
    "y1",
    "y2",
    "yChannelSelector",
    "z",
    "zoomAndPan",
    "accent-height",
    "alignment-baseline",
    "arabic-form",
    "baseline-shift",
    "cap-height",
    "clip-path",
    "clip-rule",
    "color-interpolation",
    "color-interpolation-filters",
    "color-profile",
    "color-rendering",
    "dominant-baseline",
    "enable-background",
    "fill-opacity",
    "fill-rule",
    "flood-color",
    "flood-opacity",
    "font-size-adjust",
    "font-stretch",
    "font-style",
    "font-variant",
    "font-weight",
    "glyph-name",
    "glyph-orientation-horizontal",
    "glyph-orientation-vertical",
    "horiz-adv-x",
    "horiz-origin-x",
    "image-rendering",
    "letter-spacing",
    "lighting-color",
    "marker-end",
    "marker-mid",
    "marker-start",
    "overline-position",
    "overline-thickness",
    "panose-1",
    "paint-order",
    "pointer-events",
    "rendering-intent",
    "shape-rendering",
    "stop-color",
    "stop-opacity",
    "strikethrough-position",
    "strikethrough-thickness",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-opacity",
    "stroke-width",
    "text-anchor",
    "text-decoration",
    "text-rendering",
    "underline-position",
    "underline-thickness",
    "unicode-bidi",
    "unicode-range",
    "units-per-em",
    "v-alphabetic",
    "v-hanging",
    "v-ideographic",
    "v-mathematical",
    "vector-effect",
    "vert-adv-y",
    "vert-origin-x",
    "vert-origin-y",
    "word-spacing",
    "writing-mode",
    "x-height",
    "xml:base",
    "xml:lang",
    "xml:space",
    "in",
    "type",
    "actuate",
    "arcrole",
    "href",
    "role",
    "show",
    "title",
    "type",
];
