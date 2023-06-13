//! Provides macro for creating functions for tags

macro_rules! declare_svg_tags{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment!{
                concat!("Creates an svg [",stringify!($name),"](/https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",stringify!($name),") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG>(attrs: impl IntoIterator<Item = $crate::vdom::Attribute<MSG>>, children: impl IntoIterator<Item = $crate::vdom::Node<MSG>>) -> $crate::vdom::Node<MSG>
                    {
                        $crate::svg::svg_element(stringify!($name), attrs, children)
                }
            }
         )*
    };

    ( $(
         $(#[$attr:meta])*
         $name:ident => $tagname:tt;
       )*
     ) => {
        $(
            doc_comment!{
                concat!("Creates an svg [",$tagname,"](/https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",$tagname,") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG>(attrs: impl IntoIterator<Item = $crate::vdom::Attribute<MSG>>, children: impl IntoIterator<Item = $crate::vdom::Node<MSG>>) -> $crate::vdom::Node<MSG>
                    {
                        $crate::svg::svg_element($tagname, attrs, children)
                 }
            }
         )*
    }

}

/// declare common svg tags that is not in conflict with the html tags
/// at the same time this also fills the SVG_TAGS const with all the svg tags
macro_rules! declare_common_svg_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        pub(crate) mod commons {
            declare_svg_tags! { $($name;)* }
        }


        #[cfg(feature = "with-lookup")]
        /// These are the commonly used svg tags such as rect, circle, path, arc, ..etc.
        pub const SVG_TAGS: &[&'static str] = &[ $(stringify!($name),)* ];

    };
}

/// declare svg tags, at the same time this also
/// fills up the SVG_TAGS_SPECIAL const with the svg tags that are not
/// regular identifiers
macro_rules! declare_svg_tags_special{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        declare_svg_tags!{ $($name=>$attribute;)*}

        #[cfg(feature = "with-lookup")]
        /// These are svg tags which the tags are non proper rust identifier, so they are handled
        /// differently
        pub const SVG_TAGS_SPECIAL:&[(&'static str,&'static str)] = &[$((stringify!($name),$attribute),)*];
    }
}

macro_rules! declare_svg_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_svg_tags!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        /// These are collection of svg tags that are non commonly used put together in this
        /// collection so as not to create import conflicts with the common tags
        pub const SVG_TAGS_NON_COMMON:&[&'static str] = &[$(stringify!($name),)*];
    }
}

declare_common_svg_tags_and_macro! {
    animate;
    animateMotion;
    animateTransform;
    circle;
    clipPath;
    defs;
    desc;
    discard;
    ellipse;
    feBlend;
    feColorMatrix;
    feComponentTransfer;
    feComposite;
    feConvolveMatrix;
    feDiffuseLighting;
    feDisplacementMap;
    feDistantLight;
    feDropShadow;
    feFlood;
    feFuncA;
    feFuncB;
    feFuncG;
    feFuncR;
    feGaussianBlur;
    feImage;
    feMerge;
    feMergeNode;
    feMorphology;
    feOffset;
    fePointLight;
    feSpecularLighting;
    feSpotLight;
    feTile;
    feTurbulence;
    filter;
    foreignObject;
    g;
    hatch;
    hatchpath;
    image;
    line;
    linearGradient;
    marker;
    mask;
    mesh;
    meshgradient;
    meshpatch;
    meshrow;
    metadata;
    mpath;
    path;
    pattern;
    polygon;
    polyline;
    radialGradient;
    rect;
    set;
    solidcolor;
    stop;
    svg;
    switch;
    symbol;
    textPath;
    tspan;
    unknown;
    view;
}
declare_svg_tags_special! {
    color_profile => "color-profile";
    r#use => "use";
}

// These are non-common tags
// which the users need to explicitly import using
// svg::tags::style, svg::tags::text, svg::tags::title, etc
//
declare_svg_tags_non_common! {
    script;     //> svg::tags::script   conflicts with > html::tags::script
    style;      //> svg::tags::style    conflicts with > html::attributes::style
    text;       //> svg::tags::text     conflicts with > html::text
    a;          //> svg::tags::a        conflicts with > html::tags::a
    title;      //> svg::tags::title    conflicts with > html::attributes::title
}
