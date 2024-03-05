//! provides function and macro for html units such as px, %, em, etc.

use crate::vdom::Value;
pub use fns::{rgb, rgba, rotate};

mod fns;

#[inline]
fn unit(unit_name: &str, v: impl Into<Value>) -> String {
    let value: Value = v.into();
    match value {
        Value::Vec(values) => values
            .into_iter()
            .map(|v| format!("{}{}", Into::<Value>::into(v), unit_name))
            .collect::<Vec<_>>()
            .join(" "),
        _ => {
            format!("{}{}", value, unit_name)
        }
    }
}

macro_rules! declare_units{
    (  $(
            $(#[$attr:meta])*
            $name:ident;
        )*
    ) => {
        $(
            $(#[$attr])*
            ///
            /// [MDN reference](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units)
            pub fn $name(v: impl Into<Value>) -> String {
                unit(stringify!($name), v)
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
            ///
            /// [MDN reference](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units)
            pub fn $name(v: impl Into<Value>) -> String {
                  unit($unit, v)
            }
        )*
    }
}

declare_units! {

    /// pixels (1px = 1/96th of 1in)
    ///
    /// a helper function which append `px` into a value
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10px", px(10));
    /// ```
    px;
    /// 1q is equivalent to 1/40th of 1cm.
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10q", q(10));
    /// ```
    q;
    /// milimeters
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10mm", mm(10));
    /// ```
    mm;
    /// centimeters
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10cm", cm(10));
    /// ```
    cm;
    /// points (1pt = 1/72 of 1in)
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10pt", pt(10));
    /// ```
    pt;
    /// picas (1pc = 12 pt)
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10pc", pc(10));
    /// ```
    pc;
    /// Relative to the font-size of the element (2em means 2 times the size of the current font)
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10em", em(10));
    /// ```
    em;
    /// Relative to the x-height of the current font (rarely used)
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10ex", ex(10));
    /// ```
    ex;
    /// Relative to the width of the "0" (zero)
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10ch", ch(10));
    /// ```
    ch;
    /// Relative to font-size of the root element
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10rem", rem(10));
    /// ```
    rem;
    /// Relative to 1% of the width of the viewport*
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10vw", vw(10));
    /// ```
    vw;
    /// Relative to 1% of the height of the viewport*
    ///
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10vh", vh(10));
    /// ```
    vh;
}

declare_units! {
    /// inches (1in = 96px = 2.54cm)
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10in", r#in(10))
    /// ```
    r#in => "in";
    /// percentage
    /// Example:
    /// ```rust
    /// use sauron::html::units::*;
    ///
    /// assert_eq!("10%", percent(10))
    /// ```
    percent => "%";
}

// angle units
declare_units! {
    /// Represent an angle in degrees
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
    deg;
    /// Represent an angle in radians
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
    rad;
    /// Represents an angle in gradians
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
    grad;
    /// Represents an angle in a number of turns. One full circle is 1turn.
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
    turn;
}

// time units
declare_units! {
   /// Represents a time in seconds.
   /// https://developer.mozilla.org/en-US/docs/Web/CSS/time
   s;
   /// Represents a time in milliseconds.
   /// https://developer.mozilla.org/en-US/docs/Web/CSS/time
   ms;
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
