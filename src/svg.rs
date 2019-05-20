pub use sauron_vdom::builder::{
    attr,
    element,
    element_ns,
};

pub mod attributes;

pub(in crate) const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

macro_rules! declare_svg_tags{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            pub fn $name<A, C,MSG>(attrs: A, children: C) -> $crate::Node<MSG>
                where C: AsRef<[$crate::Node<MSG>]>,
                      A: AsRef<[$crate::Attribute<MSG>]>,
                      MSG: Clone,
                {
                    $crate::html::html_element_ns(stringify!($name), SVG_NAMESPACE, attrs, children)
                }
         )*
    };

    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            pub fn $name<A, C,MSG>(attrs: A, children: C) -> $crate::Node<MSG>
                where C: AsRef<[$crate::Node<MSG>]>,
                      A: AsRef<[$crate::Attribute<MSG>]>,
                      MSG: Clone,
                {
                    $crate::html::html_element_ns($attribute, SVG_NAMESPACE, attrs, children)
                }
         )*
    }

}

declare_svg_tags! {
    a;
    animate;
    #[allow(non_snake_case)]
    animateMotion;
    #[allow(non_snake_case)]
    animateTransform;
    circle;
    #[allow(non_snake_case)]
    clipPath;
    defs;
    desc;
    discard;
    ellipse;
    #[allow(non_snake_case)]
    feBlend;
    #[allow(non_snake_case)]
    feColorMatrix;
    #[allow(non_snake_case)]
    feComponentTransfer;
    #[allow(non_snake_case)]
    feComposite;
    #[allow(non_snake_case)]
    feConvolveMatrix;
    #[allow(non_snake_case)]
    feDiffuseLighting;
    #[allow(non_snake_case)]
    feDisplacementMap;
    #[allow(non_snake_case)]
    feDistantLight;
    #[allow(non_snake_case)]
    feDropShadow;
    #[allow(non_snake_case)]
    feFlood;
    #[allow(non_snake_case)]
    feFuncA;
    #[allow(non_snake_case)]
    feFuncB;
    #[allow(non_snake_case)]
    feFuncG;
    #[allow(non_snake_case)]
    feFuncR;
    #[allow(non_snake_case)]
    feGaussianBlur;
    #[allow(non_snake_case)]
    feImage;
    #[allow(non_snake_case)]
    feMerge;
    #[allow(non_snake_case)]
    feMergeNode;
    #[allow(non_snake_case)]
    feMorphology;
    #[allow(non_snake_case)]
    feOffset;
    #[allow(non_snake_case)]
    fePointLight;
    #[allow(non_snake_case)]
    feSpecularLighting;
    #[allow(non_snake_case)]
    feSpotLight;
    #[allow(non_snake_case)]
    feTile;
    #[allow(non_snake_case)]
    feTurbulence;
    filter;
    #[allow(non_snake_case)]
    foreignObject;
    g;
    hatch;
    hatchpath;
    image;
    line;
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
    radialGradient;
    rect;
    script;
    set;
    solidcolor;
    stop;
    svg;
    switch;
    symbol;
    text;
    #[allow(non_snake_case)]
    textPath;
    title;
    tspan;
    unknown;
    view;
}

declare_svg_tags! {
    color_profile => "color-profile";
    r#use => "use";
}
