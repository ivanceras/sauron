//! provides function and macro for html units such as px, %, em, etc.
use crate::prelude::Value;

macro_rules! declare_units{
    (  $(
            $(#[$attr:meta])*
            $name:ident;
        )*
    ) => {
        $(
            $(#[$attr])*
            pub fn $name<V>(v: V) -> String
            where V: Into<Value>,
                  {
                      format!("{}{}", v.into(), stringify!($name))
                  }
        )*
    };
    (
        $(
            $(#[$attr:meta])*
            $name:ident => $unit:tt;
         )*
    ) => {
        $(
            $(#[$attr])*
            pub fn $name<V>(v: V) -> String
            where V: Into<Value>,
                  {
                      format!("{}{}", v.into(), $unit)
                  }
        )*
    }
}

declare_units! {

    /// pixels (1px = 1/96th of 1in)
    ///
    /// a helper function which append `px` into a value
    /// Example:
    /// ```ignore
    /// style("width", px(100))
    /// ```
    px;
    /// 1q is equivalent to 1/40th of 1cm.
    q;
    /// milimeters
    mm;
    /// centimeters
    cm;
    /// points (1pt = 1/72 of 1in)
    pt;
    /// picas (1pc = 12 pt)
    pc;
    ///  	Relative to the font-size of the element (2em means 2 times the size of the current font)
    em;
    /// Relative to the x-height of the current font (rarely used)
    ex;
    /// Relative to the width of the "0" (zero)
    ch;
    /// Relative to font-size of the root element
    rem;
    /// Relative to 1% of the width of the viewport*
    vw;
    /// Relative to 1% of the height of the viewport*
    vh;
}

declare_units! {
    /// inches (1in = 96px = 2.54cm)
    r#in => "in";
    /// percentage
    percent => "%";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units() {
        assert_eq!(px(1), "1px");
        assert_eq!(mm(1), "1mm");
        assert_eq!(cm(2), "2cm");
        assert_eq!(pt(5), "5pt");
        assert_eq!(pc(5), "5pc");
        assert_eq!(r#in(2.5), "2.5in");
        assert_eq!(ch(1), "1ch");
    }
}
