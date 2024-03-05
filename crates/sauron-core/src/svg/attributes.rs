//! provides functions and macros for building svg attributes
use crate::html::attributes::Value;
use crate::vdom::AttributeValue;
pub use commons::*;
use crate::vdom::{attr, attr_ns};
pub use special::*;

pub(crate) const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

macro_rules! declare_xlink_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            /// creates a function where the function name is the attribute name of the svg element
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<V, MSG>(v: V) -> crate::vdom::Attribute<MSG>
                where V: Into<Value>,
                {
                    attr_ns(Some(XLINK_NAMESPACE), $attribute, AttributeValue::from(v.into()))
                }
         )*

        #[cfg(feature = "with-lookup")]
        /// Svg attributes with xlink namespace
        pub const SVG_ATTRS_XLINK:&[(&'static str,&'static str)] = &[$((stringify!($name),$attribute),)*];
    }
}

/// declare svg attributes, at the same time fill up the
/// SVG_ATTR const with all the common svg attributes
macro_rules! declare_svg_attributes{
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_attributes!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        /// These are most commonly used svg attributes
        pub const SVG_ATTRS:&[&'static str] = &[$(stringify!($name),)*];
    }
}

macro_rules! declare_svg_attributes_non_common{
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_attributes!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        /// These are most commonly used svg attributes
        pub const SVG_ATTRS_NON_COMMON:&[&'static str] = &[$(stringify!($name),)*];
    }
}

macro_rules! declare_svg_attributes_special{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        declare_attributes!{ $($name => $attribute;)*}

        #[cfg(feature = "with-lookup")]
        /// These are svg attributes with names that are non proper rust identifier therefore they
        /// are handled differently. ie: (color-profile, accent-height, etc)
        pub const SVG_ATTRS_SPECIAL:&[(&'static str,&'static str)] = &[$((stringify!($name),$attribute),)*];
    }
}

/// common svg attributes
pub mod commons {
    use super::*;

    // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
    // complete list svg attributes
    declare_svg_attributes! {
        accumulate;
        additive;
        alphabetic;
        amplitude;
        ascent;
        azimuth;
        bbox;
        begin;
        bias;
        by;
        clip;
        cursor;
        cx;
        cy;
        d;
        decelerate;
        descent;
        direction;
        display;
        divisor;
        dur;
        dx;
        dy;
        elevation;
        end;
        exponent;
        fill;
        format;
        from;
        fr;
        fx;
        fy;
        g1;
        g2;
        hanging;
        ideographic;
        in2;
        intercept;
        k;
        k1;
        k2;
        k3;
        k4;
        kerning;
        local;
        mathematical;
        mode;
        offset;
        opacity;
        operator;
        order;
        orient;
        orientation;
        origin;
        overflow;
        points;
        r;
        radius;
        restart;
        result;
        rotate;
        rx;
        ry;
        scale;
        seed;
        slope;
        spacing;
        speed;
        stemh;
        stemv;
        string;
        stroke;
        to;
        transform;
        u1;
        u2;
        unicode;
        values;
        version;
        visibility;
        widths;
        x;
        x1;
        x2;
        xmlns;
        y;
        y1;
        y2;
        z;
    }
}

declare_svg_attributes_non_common! {
        color; //conflicts with html::attributes::color
        filter; //conflicts with svg::filter
        height; //conflicts with html::attributes::height
        href; //conflicts with html::attributes::href;
        hreflang;
        lang;
        mask; //conflicts with svg::mask
        max;
        media;
        method;
        min;
        name;
        ping;
        rel;
        tabindex;
        target;
        width;
}

/// special svg attributes
pub mod special {
    use super::*;
    // These are attributes that is exposed in such a way that is consistent to rust conventions
    // This includes exposing the following:
    // - reserved keywords
    // - kebab-case attributes
    // - namespaced/colon separated attributes such as xml::lang
    // - camelCase attributes
    declare_svg_attributes_special! {

        ///////////////////////////////
        // rust reserved keywords that are svg attributes
        ///////////////////////////////
        r#in => "in";

        /////////////////////////////////
        // kebab-case svg attributes
        /////////////////////////////////
        accent_height => "accent-height";
        alignment_baseline => "alignment-baseline";
        arabic_form => "arabic-form";
        baseline_shift => "baseline-shift";
        cap_height => "cap-height";
        clip_path => "clip-path";
        clip_rule => "clip-rule";
        color_interpolation => "color-interpolation";
        color_interpolation_filters => "color-interpolation-filters";
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

        ////////////////////////////////////////
        // namespaced svg attributes
        ////////////////////////////////////////
        xml_base => "xml:base";
        xml_lang => "xml:lang";
        xml_space => "xml:space";
        xmlns_xlink => "xmlns:xlink";

        /////////////////////////////////
        // camelCase svg attributes
        /////////////////////////////////
        allow_reorder => "allowReorder";
        attribute_name => "attributeName";
        attribute_type => "attributeType";
        auto_reverse => "autoReverse";
        base_frequency => "baseFrequency";
        base_profile => "baseProfile";
        calc_mode => "calcMode";
        clip_path_units => "clipPathUnits";
        content_script_type => "contentScriptType";
        content_style_type => "contentStyleType";
        diffuse_constant => "diffuseConstant";
        edge_mode => "edgeMode";
        external_resources_required => "externalResourcesRequired";
        filter_res => "filterRes";
        filter_units => "filterUnits";
        glyph_ref => "glyphRef";
        gradient_transform => "gradientTransform";
        gradient_units => "gradientUnits";
        kernel_matrix => "kernelMatrix";
        kernel_unit_length => "kernelUnitLength";
        key_points => "keyPoints";
        key_splines => "keySplines";
        key_times => "keyTimes";
        length_adjust => "lengthAdjust";
        limiting_coneAngle => "limitingConeAngle";
        marker_height => "markerHeight";
        marker_units => "markerUnits";
        marker_width => "markerWidth";
        mask_content_units => "maskContentUnits";
        mask_units => "maskUnits";
        num_octaves => "numOctaves";
        path_length => "pathLength";
        pattern_content_units => "patternContentUnits";
        pattern_transform => "patternTransform";
        pattern_units => "patternUnits";
        points_at_x => "pointsAtX";
        points_at_y => "pointsAtY";
        points_at_z => "pointsAtZ";
        preserve_alpha => "preserveAlpha";
        preserve_aspect_ratio => "preserveAspectRatio";
        primitive_units => "primitiveUnits";
        referrer_policy => "referrerPolicy";
        ref_x => "refX";
        ref_y => "refY";
        repeat_count => "repeatCount";
        repeat_dur => "repeatDur";
        required_extensions => "requiredExtensions";
        required_features => "requiredFeatures";
        specular_constant => "specularConstant";
        specular_exponent => "specularExponent";
        spread_method => "spreadMethod";
        start_offset => "startOffset";
        std_deviation => "stdDeviation";
        stitch_tiles => "stitchTiles";
        surface_scale => "surfaceScale";
        system_language => "systemLanguage";
        table_values => "tableValues";
        target_x => "targetX";
        target_y => "targetY";
        text_length => "textLength";
        view_box => "viewBox";
        view_target => "viewTarget";
        x_channel_selector => "xChannelSelector";
        y_channel_selector => "yChannelSelector";
        zoom_and_pan => "zoomAndPan";
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
}
