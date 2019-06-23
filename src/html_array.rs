pub use sauron_vdom::builder::{
    attr,
    on,
    text,
};

macro_rules! declare_nicer_tags {

    ( $(
         $(#[$attr:meta])*
         $name:ident => $M:tt,$N:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<A, C,MSG>(attrs: A, children: C) -> $crate::Node<MSG>
                where C: AsRef<[$crate::Node<MSG>]>,
                      A: AsRef<[$crate::Attribute<MSG>]>,
                      MSG: Clone,
                {
                    $crate::html::$name(attrs.as_ref().to_vec(), children.as_ref().to_vec())
                }

            /*
            pub fn $name<MSG, const $M: usize, const $N: usize>(attrs: [$crate::Attribute<MSG>;$M], children: [$crate::Node<MSG>;$N]) -> $crate::Node<MSG>
                where
                      MSG: Clone,
                {
                    $crate::html::$name(attrs.to_vec(), children.to_vec())
                }
            */
         )*
    }
}

declare_nicer_tags! {
    base => M,N;
    head => M,N;
    link => M,N;
    meta => M,N;
    body => M,N;
    address => M,N;
    article => M,N;
    aside => M,N;
    footer => M,N;
    header => M,N;
    h1 => M,N;
    h2 => M,N;
    h3 => M,N;
    h4 => M,N;
    h5 => M,N;
    h6 => M,N;
    hgroup => M,N;
    main => M,N;
    nav => M,N;
    section => M,N;
    blockquote => M,N;
    dd => M,N;
    div => M,N;
    dl => M,N;
    dt => M,N;
    figcaption => M,N;
    figure => M,N;
    hr => M,N;
    li => M,N;
    ol => M,N;
    p => M,N;
    pre => M,N;
    ul => M,N;
    a => M,N;
    abbr => M,N;
    b => M,N;
    bdi => M,N;
    bdo => M,N;
    br => M,N;
    cite => M,N;
    code => M,N;
    data => M,N;
    dfn => M,N;
    em => M,N;
    i => M,N;
    kbd => M,N;
    mark => M,N;
    q => M,N;
    rb => M,N;
    rp => M,N;
    rt => M,N;
    rtc => M,N;
    ruby => M,N;
    s => M,N;
    samp => M,N;
    small => M,N;
    span => M,N;
    strong => M,N;
    sub => M,N;
    sup => M,N;
    time => M,N;
    u => M,N;
    var => M,N;
    wbr => M,N;
    area => M,N;
    audio => M,N;
    img => M,N;
    map => M,N;
    track => M,N;
    video => M,N;
    embed => M,N;
    iframe => M,N;
    object => M,N;
    param => M,N;
    picture => M,N;
    source => M,N;
    canvas => M,N;
    noscript => M,N;
    script => M,N;
    del => M,N;
    ins => M,N;
    caption => M,N;
    col => M,N;
    colgroup => M,N;
    table => M,N;
    tbody => M,N;
    td => M,N;
    tfoot => M,N;
    th => M,N;
    thead => M,N;
    tr => M,N;
    button => M,N;
    datalist => M,N;
    fieldset => M,N;
    form => M,N;
    input => M,N;
    label => M,N;
    legend => M,N;
    meter => M,N;
    optgroup => M,N;
    option => M,N;
    output => M,N;
    progress => M,N;
    select => M,N;
    textarea => M,N;
    details => M,N;
    dialog => M,N;
    menu => M,N;
    menuitem => M,N;
    summary => M,N;
    template => M,N;
}
