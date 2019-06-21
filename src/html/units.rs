use sauron_vdom::Value;

macro_rules! declare_units{
    (  $(
            $(#[$attr:meta])*
            $name:ident;
        )*
    ) => {
        $(
            $(#[$attr])*
            pub fn $name<V>(v: V) -> String
            where V: Into<Value> + Clone,
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
            where V: Into<Value> + Clone,
                  {
                      format!("{}{}", v.into(), $unit)
                  }
        )*
    }
}

declare_units! {

    /// a helper function which append `px` into a value
    /// Example:
    /// ```ignore
    /// style("width", px(100))
    /// ```
    px;
    q;
    mm;
    cm;
    pt;
    pc;
    em;
    ex;
    ch;
    rem;
    vw;
    vh;
}

declare_units! {
    r#in => "in";
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
