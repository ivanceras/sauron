pub use sauron_vdom::builder::attr;
use sauron_vdom::{builder::Attribute,
                  Value};

// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
// complete list svg attributes
declare_attributes! {
    accumulate;
    additive;
    #[allow(non_snake_case)]
    allowReorder;
    alphabetic;
    amplitude;
    ascent;
    #[allow(non_snake_case)]
    attributeName;
    #[allow(non_snake_case)]
    attributeType;
    #[allow(non_snake_case)]
    autoReverse;
    azimuth;
    #[allow(non_snake_case)]
    baseFrequency;
    #[allow(non_snake_case)]
    baseProfile;
    bbox;
    begin;
    bias;
    by;
    #[allow(non_snake_case)]
    calcMode;
    class;
    clip;
    #[allow(non_snake_case)]
    clipPathUnits;
    color;
    #[allow(non_snake_case)]
    contentScriptType;
    #[allow(non_snake_case)]
    contentStyleType;
    cursor;
    cx;
    cy;
    d;
    decelerate;
    descent;
    #[allow(non_snake_case)]
    diffuseConstant;
    direction;
    display;
    divisor;
    dur;
    dx;
    dy;
    #[allow(non_snake_case)]
    edgeMode;
    elevation;
    end;
    exponent;
    #[allow(non_snake_case)]
    externalResourcesRequired;
    fill;
    filter;
    #[allow(non_snake_case)]
    filterRes;
    #[allow(non_snake_case)]
    filterUnits;
    format;
    from;
    fr;
    fx;
    fy;
    g1;
    g2;
    #[allow(non_snake_case)]
    glyphRef;
    #[allow(non_snake_case)]
    gradientTransform;
    #[allow(non_snake_case)]
    gradientUnits;
    hanging;
    height;
    href;
    hreflang;
    id;
    ideographic;
    in2;
    intercept;
    k;
    k1;
    k2;
    k3;
    k4;
    #[allow(non_snake_case)]
    kernelMatrix;
    #[allow(non_snake_case)]
    kernelUnitLength;
    kerning;
    #[allow(non_snake_case)]
    keyPoints;
    #[allow(non_snake_case)]
    keySplines;
    #[allow(non_snake_case)]
    keyTimes;
    lang;
    #[allow(non_snake_case)]
    lengthAdjust;
    #[allow(non_snake_case)]
    limitingConeAngle;
    local;
    #[allow(non_snake_case)]
    markerHeight;
    #[allow(non_snake_case)]
    markerUnits;
    #[allow(non_snake_case)]
    markerWidth;
    mask;
    #[allow(non_snake_case)]
    maskContentUnits;
    #[allow(non_snake_case)]
    maskUnits;
    mathematical;
    max;
    media;
    method;
    min;
    mode;
    name;
    #[allow(non_snake_case)]
    numOctaves;
    offset;
    opacity;
    operator;
    order;
    orient;
    orientation;
    origin;
    overflow;
    path;
    #[allow(non_snake_case)]
    pathLength;
    #[allow(non_snake_case)]
    patternContentUnits;
    #[allow(non_snake_case)]
    patternTransform;
    #[allow(non_snake_case)]
    patternUnits;
    ping;
    points;
    #[allow(non_snake_case)]
    pointsAtX;
    #[allow(non_snake_case)]
    pointsAtY;
    #[allow(non_snake_case)]
    pointsAtZ;
    #[allow(non_snake_case)]
    preserveAlpha;
    #[allow(non_snake_case)]
    preserveAspectRatio;
    #[allow(non_snake_case)]
    primitiveUnits;
    r;
    radius;
    #[allow(non_snake_case)]
    referrerPolicy;
    #[allow(non_snake_case)]
    refX;
    #[allow(non_snake_case)]
    refY;
    rel;
    #[allow(non_snake_case)]
    repeatCount;
    #[allow(non_snake_case)]
    repeatDur;
    #[allow(non_snake_case)]
    requiredExtensions;
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
    specularConstant;
    #[allow(non_snake_case)]
    specularExponent;
    speed;
    #[allow(non_snake_case)]
    spreadMethod;
    #[allow(non_snake_case)]
    startOffset;
    #[allow(non_snake_case)]
    stdDeviation;
    stemh;
    stemv;
    #[allow(non_snake_case)]
    stitchTiles;
    string;
    stroke;
    style;
    #[allow(non_snake_case)]
    surfaceScale;
    #[allow(non_snake_case)]
    systemLanguage;
    tabindex;
    #[allow(non_snake_case)]
    tableValues;
    target;
    #[allow(non_snake_case)]
    targetX;
    #[allow(non_snake_case)]
    targetY;
    #[allow(non_snake_case)]
    textLength;
    to;
    transform;
    u1;
    u2;
    unicode;
    values;
    version;
    #[allow(non_snake_case)]
    viewBox;
    #[allow(non_snake_case)]
    viewTarget;
    visibility;
    width;
    widths;
    x;
    x1;
    x2;
    #[allow(non_snake_case)]
    xChannelSelector;
    xmlns;
    y;
    y1;
    y2;
    #[allow(non_snake_case)]
    yChannelSelector;
    z;
    #[allow(non_snake_case)]
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
    font_family => "font-family";
    font_size => "font-size";
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
    xlink_actuate => "xlink:actuate";
    xlink_arcrole => "xlink:arcrole";
    xlink_href => "xlink:href";
    xlink_role => "xlink:role";
    xlink_show => "xlink:show";
    xlink_title => "xlink:title";
    xlink_type => "xlink:type";
    xml_base => "xml:base";
    xml_lang => "xml:lang";
    xml_space => "xml:space";
}

declare_attributes! {
    r#in => "in";
    r#type => "type";
}

//TODO: add the rest of the attributes that are used in svg elements
