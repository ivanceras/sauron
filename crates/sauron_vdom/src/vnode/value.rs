use std::fmt;

/// Value is an abstraction of the values used in the actual
/// backend. Html and gtk-rs have different set of compatible values
/// therefore a need for a storage of these intermediate value is needed
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Str(&'static str),
    String(String),
    Vec(Vec<Value>),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
    Bytes(Vec<u8>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(ref v) => Some(&v),
            Value::Str(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Bool(_) => None,
            Value::String(_v) => None,
            Value::Str(_v) => None,
            Value::Vec(_v) => None,
            Value::U8(v) => Some(f64::from(*v)),
            Value::U16(v) => Some(f64::from(*v)),
            Value::U32(v) => Some(f64::from(*v)),
            Value::U64(v) => Some(*v as f64),
            Value::U128(v) => Some(*v as f64),
            Value::Usize(v) => Some(*v as f64),
            Value::I8(v) => Some(f64::from(*v)),
            Value::I16(v) => Some(f64::from(*v)),
            Value::I32(v) => Some(f64::from(*v)),
            Value::I64(v) => Some(*v as f64),
            Value::I128(v) => Some(*v as f64),
            Value::Isize(v) => Some(*v as f64),
            Value::F32(v) => Some(f64::from(*v)),
            Value::F64(v) => Some(*v),
            Value::Bytes(_) => None,
        }
    }

    /// If this is Value::Vec variant, append the new value
    /// otherwise, turn this value into Value::Vec(Vec<Value>) variant
    /// and append the new value.
    pub fn append(&mut self, new_value: Value) {
        match self {
            Value::Vec(values) => {
                values.push(new_value);
            }
            _ => {
                *self = Value::Vec(vec![self.clone(), new_value]);
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Vec(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::U8(v) => write!(f, "{}", v),
            Value::U16(v) => write!(f, "{}", v),
            Value::U32(v) => write!(f, "{}", v),
            Value::U64(v) => write!(f, "{}", v),
            Value::U128(v) => write!(f, "{}", v),
            Value::Usize(v) => write!(f, "{}", v),
            Value::I8(v) => write!(f, "{}", v),
            Value::I16(v) => write!(f, "{}", v),
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::I128(v) => write!(f, "{}", v),
            Value::Isize(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
            Value::Bytes(_) => panic!("bytes should not be displayed"),
        }
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::String(v.to_string())
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Bytes(v.to_vec())
    }
}

impl From<&Vec<u8>> for Value {
    fn from(v: &Vec<u8>) -> Self {
        Value::Bytes(v.to_owned())
    }
}

macro_rules! impl_from {
    ($ty:ty => $variant:ident) => {
        impl From<$ty> for Value {
            fn from(f: $ty) -> Self {
                Value::$variant(f)
            }
        }
    };
    (($($T:ident),*) => $($n:tt),*) => {
        impl<$($T),*>From<($($T),*)> for Value
            where
                $($T: Into<Value>),*
        {
            fn from(v: ($($T),*)) -> Self {
                Value::Vec(vec![$(v.$n.into()),*])
            }
        }
    };

    ([$T:ident;$n:tt]) => {
        impl<T> From<[T; $n]> for Value
        where
            T: Into<Value> + Clone,
        {
            fn from(v: [T; $n]) -> Self {
                Value::Vec(
                    v.iter()
                        .map(|i| i.to_owned().into())
                        .collect::<Vec<Value>>(),
                )
            }
        }
    }
}

impl_from!(bool => Bool);
impl_from!(String => String);
impl_from!(&'static str => Str);
impl_from!(u8 => U8);
impl_from!(u16 => U16);
impl_from!(u32 => U32);
impl_from!(u64 => U64);
impl_from!(u128 => U128);
impl_from!(usize => Usize);
impl_from!(i8 => I8);
impl_from!(i16 => I16);
impl_from!(i32 => I32);
impl_from!(i64 => I64);
impl_from!(i128 => I128);
impl_from!(isize => Isize);
impl_from!(f32 => F32);
impl_from!(f64 => F64);
impl_from!(Vec<u8> => Bytes);

impl_from!((T, U) => 0,1);
impl_from!((T, U, V) => 0,1,2);
impl_from!((T, U, V,X) => 0,1,2,3);
impl_from!((T, U, V,X,Z) => 0,1,2,3,4);

impl_from!([T; 1]);
impl_from!([T; 2]);
impl_from!([T; 3]);
impl_from!([T; 4]);
impl_from!([T; 5]);
impl_from!([T; 6]);
impl_from!([T; 7]);
impl_from!([T; 8]);
impl_from!([T; 9]);
impl_from!([T; 10]);
impl_from!([T; 11]);
impl_from!([T; 12]);

#[cfg(test)]
mod tests {
    use crate::{
        builder::{attr, element},
        Node,
    };

    #[test]
    fn tuple_value() {
        let line: Node<&'static str, &'static str, (), ()> =
            element("line", vec![attr("stroke-dasharray", (10, 20))], vec![]);
        let expected = "<line stroke-dasharray=\"10 20\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in tuple should be flatten to string"
        );

        let line: Node<&'static str, &'static str, (), ()> =
            element("line", vec![attr("transition", ("opacity", 1))], vec![]);
        let expected = "<line transition=\"opacity 1\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in tuple should be flatten to string"
        );

        let line: Node<&'static str, &'static str, (), ()> = element(
            "line",
            vec![attr("transition", ("opacity", 1, "linear"))],
            vec![],
        );
        let expected = "<line transition=\"opacity 1 linear\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in tuple should be flatten to string"
        );

        let line: Node<&'static str, &'static str, (), ()> = element(
            "line",
            vec![attr("transition", ("opacity", 1, "linear", true))],
            vec![],
        );
        let expected = "<line transition=\"opacity 1 linear true\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in tuple should be flatten to string"
        );
    }

    #[test]
    fn array_value() {
        let line: Node<&'static str, &'static str, (), ()> =
            element("line", vec![attr("stroke-dasharray", [10, 20])], vec![]);
        let expected = "<line stroke-dasharray=\"10 20\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in array should be flatten to string"
        );

        let line: Node<&'static str, &'static str, (), ()> = element(
            "line",
            vec![attr("stroke-dasharray", [10, 20, 30, 40])],
            vec![],
        );
        let expected = "<line stroke-dasharray=\"10 20 30 40\"></line>";
        assert_eq!(
            format!("{}", line),
            expected,
            "The value in array should be flatten to string"
        );
    }
}
