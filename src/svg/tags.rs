macro_rules! declare_svg_tags{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG>(attrs: Vec<$crate::Attribute<MSG>>, children: Vec<$crate::Node<MSG>>) -> $crate::Node<MSG>
                {
                    $crate::svg::svg_element(stringify!($name), attrs, children)
                }
         )*
    };

    ( $(
         $(#[$attr:meta])*
         $name:ident => $tagname:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG>(attrs: Vec<$crate::Attribute<MSG>>, children: Vec<$crate::Node<MSG>>) -> $crate::Node<MSG>
                {
                    $crate::svg::svg_element($tagname, attrs, children)
                }
         )*
    }

}

pub(super) mod commons {
    declare_svg_tags! {
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
        script;
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
    declare_svg_tags! {
        color_profile => "color-profile";
        r#use => "use";
    }
}

// These are non-common tags
// which the users need to explicitly import using
// svg::tags::style, svg::tags::text, svg::tags::title, etc
//
declare_svg_tags! {
    style;
    text;
    a;
    title;
}
