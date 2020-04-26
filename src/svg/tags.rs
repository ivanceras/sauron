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

// declare a tags in a macro
// Note: The $ dollar sign is explcitly pass to prevent
// rustc to attempt to expand the inner repetion of the macro
macro_rules! declare_svg_tags_macro {
    (($d:tt) $($name: ident;)*) => {
        $(

        #[macro_export]
        macro_rules! $name {

            // 000: no trailing commas
            ( [$d($att: expr),*], [$d($children: expr),*] ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };

            ///////////////////////////////////////////////////////////////
            //
            // The next code is just the same logic as the first, it is just
            // here to deal with irregular comma placement
            //
            ///////////////////////////////////////////////////////////////

            // 001: trailing commas in params only
            ( [$d($att: expr),*], [$d($children: expr),*], ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 010: trailing commas in children only
            ( [$d($att: expr),*], [$d($children: expr,)*] ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 011: trailing commas in children and params,
            ( [$d($att: expr),*], [$d($children: expr,)*], ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 100: trailing commas in attributes only
            ( [$d($att: expr,)*], [$d($children: expr),*] ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 101: trailing commas in attributes and params,
            ( [$d($att: expr,)*], [$d($children: expr,)*], ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 110: trailing commas in attributes and children
            ( [$d($att: expr,)*], [$d($children: expr,)*] ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };
            // 111: trailing commas in attributes, children, params
            ( [$d($att: expr,)*], [$d($children: expr,)*], ) => {
                $crate::svg::$name(vec![$d($att),*], vec![$d($children),*])
            };

            /////////////////////////////////////////////////
            //
            // Pass through the expression as it was with the old function call
            //
            /////////////////////////////////////////////////

            // Pass through the div(vec![], vec![])
            ( $att: expr, $children: expr ) => {
                $crate::svg::$name( $att, $children)
            };

            // Pass through the div!(vec![], vec![],) with trailing comma
            ( $att: expr, $children: expr, ) => {
                $crate::svg::$name( $att, $children)
            };
        }
        )*
    };
}

/// declare common svg tags that is not in conflict with the html tags
/// at the same time this also fills the SVG_TAGS const with all the svg tags
macro_rules! declare_common_svg_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        pub(crate) mod commons {
            declare_svg_tags! { $($name;)* }

            pub(crate) mod macros{
                declare_svg_tags_macro! {($) $($name;)* }
            }
        }


        #[cfg(feature = "with-parser")]
        pub const SVG_TAGS: [&'static str; 65] = [ $(stringify!($name),)* ];

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

        #[cfg(feature = "with-parser")]
        pub const SVG_TAGS_SPECIAL:[(&'static str,&'static str); 3] = [$((stringify!($name),$attribute),)*];
    }
}

macro_rules! declare_svg_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_svg_tags!{ $($name;)*}

        #[cfg(feature = "with-parser")]
        pub const SVG_TAGS_NON_COMMON:[&'static str;6] = [$(stringify!($name),)*];
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
    use_ => "use";
}

// These are non-common tags
// which the users need to explicitly import using
// svg::tags::style, svg::tags::text, svg::tags::title, etc
//
declare_svg_tags_non_common! {
    line; // since this conflicts with std::line! macro, std::line                > svg::tags::line
    script; // this conflicts with html::script        , html::tags::script       > svg::tags::script
    style; // conflics with html::attributes::style    , html::attributes::style  > svg::tags::style
    text; // conflicts with html::text                 , html::text               > svg::tags::text
    a;   // conflicts with html::a                     , html::tags::a            > svg::tags::a
    title;  // conflicts with html::attributes::title  , html::attributes::title  > svg::tags::title
}
