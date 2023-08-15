use std::fmt;

/// Wraps different primitive variants used as values in html
/// This is needed since html attributes can have different value types
/// such as checked(bool), name(String), tab_index(i32)
/// Note: memory size of Value is 32 bytes, in comparison String is 24 bytes
#[derive(Debug, Clone)]
pub enum Value {
    /// bool value
    Bool(bool),
    /// &'static str value
    Str(&'static str),
    /// String value
    String(String),
    /// a vec of values
    Vec(Vec<Value>),
    /// u8 value
    U8(u8),
    /// u16 value
    U16(u16),
    /// u32 value
    U32(u32),
    /// u64 value
    U64(u64),
    /// usize value
    Usize(usize),
    /// u128 value
    U128(u128),
    /// i8 value
    I8(i8),
    /// i16 value
    I16(i16),
    /// i32 value
    I32(i32),
    /// i64 value
    I64(i64),
    /// i128 value
    I128(i128),
    /// isize value
    Isize(isize),
    /// f32 value
    F32(f32),
    /// f64 value
    F64(f64),
}

impl Value {
    /// returns an &str reference if this value is `Str` or `String` variant
    /// Note: This doesn't convert other variant into str representation
    /// Use the `to_string()` for that.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(ref v) => Some(v),
            Self::Str(v) => Some(v),
            _ => None,
        }
    }

    /// returns the bool value if this a Bool variant
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// converts to f64 if the variants are numerical representation
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Bool(_) => None,
            Self::String(_v) => None,
            Self::Str(_v) => None,
            Self::Vec(_v) => None,
            Self::U8(v) => Some(f64::from(*v)),
            Self::U16(v) => Some(f64::from(*v)),
            Self::U32(v) => Some(f64::from(*v)),
            Self::U64(v) => Some(*v as f64),
            Self::U128(v) => Some(*v as f64),
            Self::Usize(v) => Some(*v as f64),
            Self::I8(v) => Some(f64::from(*v)),
            Self::I16(v) => Some(f64::from(*v)),
            Self::I32(v) => Some(f64::from(*v)),
            Self::I64(v) => Some(*v as f64),
            Self::I128(v) => Some(*v as f64),
            Self::Isize(v) => Some(*v as f64),
            Self::F32(v) => Some(f64::from(*v)),
            Self::F64(v) => Some(*v),
        }
    }

    /// converts to i32 if the variants are numerical representation
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Bool(_) => None,
            Self::String(_v) => None,
            Self::Str(_v) => None,
            Self::Vec(_v) => None,
            Self::U8(v) => Some(i32::from(*v)),
            Self::U16(v) => Some(i32::from(*v)),
            Self::U32(v) => Some(*v as i32),
            Self::U64(v) => Some(*v as i32),
            Self::U128(v) => Some(*v as i32),
            Self::Usize(v) => Some(*v as i32),
            Self::I8(v) => Some(i32::from(*v)),
            Self::I16(v) => Some(i32::from(*v)),
            Self::I32(v) => Some(*v),
            Self::I64(v) => Some(*v as i32),
            Self::I128(v) => Some(*v as i32),
            Self::Isize(v) => Some(*v as i32),
            Self::F32(v) => Some(*v as i32),
            Self::F64(v) => Some(*v as i32),
        }
    }

    /// If this is Value::Vec variant, append the new value
    /// otherwise, turn this value into Value::Vec(Vec<Value>) variant
    /// and append the new value.
    pub fn append(&mut self, new_value: Value) {
        match self {
            Self::Vec(values) => {
                values.push(new_value);
            }
            _ => {
                *self = Value::Vec(vec![self.clone(), new_value]);
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(v), Self::Bool(o)) => v == o,
            (Self::String(v), other) => match other {
                Self::String(o) => v == o,
                Self::Str(o) => v == o,
                _ => false,
            },
            (Self::Str(v), other) => match other {
                Self::String(o) => v == o,
                Self::Str(o) => v == o,
                _ => false,
            },
            (Self::Vec(v), Self::Vec(o)) => v == o,
            (Self::U8(v), Self::U8(o)) => v == o,
            (Self::U16(v), Self::U16(o)) => v == o,
            (Self::U32(v), Self::U32(o)) => v == o,
            (Self::U64(v), Self::U64(o)) => v == o,
            (Self::U128(v), Self::U128(o)) => v == o,
            (Self::Usize(v), Self::Usize(o)) => v == o,
            (Self::I8(v), Self::I8(o)) => v == o,
            (Self::I16(v), Self::I16(o)) => v == o,
            (Self::I32(v), Self::I32(o)) => v == o,
            (Self::I64(v), Self::I64(o)) => v == o,
            (Self::I128(v), Self::I128(o)) => v == o,
            (Self::Isize(v), Self::Isize(o)) => v == o,
            (Self::F32(v), Self::F32(o)) => v == o,
            (Self::F64(v), Self::F64(o)) => v == o,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Vec(v) => {
                write!(
                    f,
                    "{}",
                    v.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Self::U8(v) => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
            Self::U128(v) => write!(f, "{}", v),
            Self::Usize(v) => write!(f, "{}", v),
            Self::I8(v) => write!(f, "{}", v),
            Self::I16(v) => write!(f, "{}", v),
            Self::I32(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
            Self::I128(v) => write!(f, "{}", v),
            Self::Isize(v) => write!(f, "{}", v),
            Self::F32(v) => write!(f, "{}", v),
            Self::F64(v) => write!(f, "{}", v),
        }
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Self::String(v.to_string())
    }
}

impl From<&'static str> for Value {
    fn from(v: &'static str) -> Self {
        Self::Str(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl<T, const N: usize> From<[T; N]> for Value
where
    T: Into<Value> + Clone,
{
    fn from(v: [T; N]) -> Self {
        Value::Vec(
            v.iter()
                .map(|i| i.to_owned().into())
                .collect::<Vec<Value>>(),
        )
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
}

impl_from!(bool => Bool);
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

impl<V0, V1> From<(V0, V1)> for Value
where
    V0: Into<Value>,
    V1: Into<Value>,
{
    fn from(values: (V0, V1)) -> Self {
        Self::Vec(vec![values.0.into(), values.1.into()])
    }
}

impl<V0, V1, V2> From<(V0, V1, V2)> for Value
where
    V0: Into<Value>,
    V1: Into<Value>,
    V2: Into<Value>,
{
    fn from(values: (V0, V1, V2)) -> Self {
        Self::Vec(vec![values.0.into(), values.1.into(), values.2.into()])
    }
}

impl<V0, V1, V2, V3> From<(V0, V1, V2, V3)> for Value
where
    V0: Into<Value>,
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
{
    fn from(values: (V0, V1, V2, V3)) -> Self {
        Self::Vec(vec![
            values.0.into(),
            values.1.into(),
            values.2.into(),
            values.3.into(),
        ])
    }
}

impl<V0, V1, V2, V3, V4> From<(V0, V1, V2, V3, V4)> for Value
where
    V0: Into<Value>,
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
{
    fn from(values: (V0, V1, V2, V3, V4)) -> Self {
        Self::Vec(vec![
            values.0.into(),
            values.1.into(),
            values.2.into(),
            values.3.into(),
            values.4.into(),
        ])
    }
}
