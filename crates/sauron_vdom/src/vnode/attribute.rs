//! attribute module
use crate::{
    Callback,
    Value,
};
use std::{
    fmt,
    fmt::Write,
};
pub use style::Style;

mod style;

/// Attributes of an element which can be any of the following
/// style, a function call, and event listener or just a plain attribute
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    /// plain attribute
    Plain(PlainAttribute<ATT>),
    /// a style attribute
    Style(Vec<Style<ATT>>),
    /// a function call such as inner_html
    FunctionCall(ATT, Value),
    /// an event listener
    EventListener(ATT, Callback<EVENT, MSG>),
}

/// These are the plain attributes of an element
#[derive(Debug, Clone, PartialEq)]
pub struct PlainAttribute<ATT>
where
    ATT: Clone,
{
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Value,
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<&'static str>,
}

impl<ATT> PlainAttribute<ATT>
where
    ATT: Clone,
{
    /// create a plain attribute with namespace
    pub fn with_namespace(
        name: ATT,
        value: Value,
        namespace: Option<&'static str>,
    ) -> Self {
        PlainAttribute {
            name,
            value,
            namespace,
        }
    }

    /// create a nice string representation of this attribute
    pub fn render(&self, buffer: &mut dyn Write) -> fmt::Result
    where
        ATT: ToString,
    {
        if let Some(_ns) = self.namespace {
            //TODO: the xlink part of this namespace should be passed by the calling function
            //Consideration, in apply patches setting the attribute
            // has to set only the name, and the xlink namespace is not
            // included
            write!(
                buffer,
                r#"xlink:{}="{}""#,
                self.name.to_string(),
                self.value
            )?;
        } else {
            write!(buffer, r#"{}="{}""#, self.name.to_string(), self.value)?;
        }
        Ok(())
    }
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    /// create a plain attribute with namespace
    pub fn with_namespace(
        name: ATT,
        value: Value,
        namespace: Option<&'static str>,
    ) -> Self {
        Attribute::Plain(PlainAttribute::with_namespace(name, value, namespace))
    }

    /// return the name of the attribute
    /// panics if called on style, since style is not named.
    /// TODO: override this on sauron crate, with style return as "style"
    pub fn name(&self) -> &ATT {
        match self {
            Attribute::Plain(plain) => &plain.name,
            Attribute::EventListener(event, _) => event,
            Attribute::FunctionCall(func, _) => func,
            Attribute::Style(_) => todo!(), //"style",
        }
    }

    /// return the namspace if there is
    pub fn namespace(&self) -> Option<&'static str> {
        match self {
            Attribute::Plain(plain) => plain.namespace.as_ref().map(|s| *s),
            _ => None,
        }
    }

    /// create an attribute from Callback
    pub fn from_callback(name: ATT, cb: Callback<EVENT, MSG>) -> Self {
        Attribute::EventListener(name, cb)
    }

    /// create an attribute from Value type
    pub fn from_value(name: ATT, value: Value) -> Self {
        Attribute::Plain(PlainAttribute {
            name,
            value,
            namespace: None,
        })
    }

    /// create an attribute from a function `name` with arguments `value`
    pub fn function_call(name: ATT, value: Value) -> Self {
        Attribute::FunctionCall(name, value)
    }

    /// create an attribute from Vec<Style>
    pub fn from_styles(styles: Vec<Style<ATT>>) -> Self {
        Attribute::Style(styles)
    }

    /// check whether this attribute is an event listener
    pub fn is_event_listener(&self) -> bool {
        match self {
            Attribute::EventListener(_, _) => true,
            _ => false,
        }
    }

    /// check whether this attribute is a value
    pub fn is_value(&self) -> bool {
        match self {
            Attribute::Plain(_) => true,
            _ => false,
        }
    }

    /// check whether this attribute is a func call value
    /// such as `inner_html` etc.
    pub fn is_function_call(&self) -> bool {
        match self {
            Attribute::FunctionCall(_, _) => true,
            _ => false,
        }
    }

    /// check whether this attribute is a style
    /// style is threated differently and with flexibility
    /// which could be merge, remove, added
    pub fn is_style(&self) -> bool {
        match self {
            Attribute::Style(_) => true,
            _ => false,
        }
    }

    /// returns a reference to the value of this attribute
    pub fn get_value(&self) -> Option<&Value> {
        match self {
            Attribute::Plain(plain) => Some(&plain.value),
            _ => None,
        }
    }

    /// return the args to the function call
    pub fn get_function_call_value(&self) -> Option<&Value> {
        match self {
            Attribute::FunctionCall(_, value) => Some(value),
            _ => None,
        }
    }

    /// return the styles if it is a style value
    pub fn get_styles(&self) -> Option<&Vec<Style<ATT>>> {
        match self {
            Attribute::Style(styles) => Some(styles),
            _ => None,
        }
    }

    /// return a mutable reference to styles if it is a style value
    pub fn get_styles_mut(&mut self) -> Option<&mut Vec<Style<ATT>>> {
        match self {
            Attribute::Style(styles) => Some(styles),
            _ => None,
        }
    }

    /// returns the reference to the callback of this attribute
    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            Attribute::EventListener(_, cb) => Some(cb),
            _ => None,
        }
    }

    /// consume the attribute and take the callback
    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        match self {
            Attribute::EventListener(_, cb) => Some(cb),
            _ => None,
        }
    }

    /// create a nice string representation of this attribute
    pub fn render(&self, buffer: &mut dyn Write) -> fmt::Result
    where
        ATT: ToString,
    {
        match self {
            Attribute::Plain(plain) => {
                plain.render(buffer)?;
            }
            Attribute::Style(styles) => {
                if !styles.is_empty() {
                    write!(buffer, "style=\"")?;
                    for style in styles {
                        write!(buffer, "{};", style)?;
                    }
                    write!(buffer, "\"")?;
                }
            }
            _ => (),
        }
        Ok(())
    }
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    /// map/transform the callback of this attribute where MSG becomes MSG2
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Attribute<ATT, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Attribute::EventListener(name, listener) => {
                Attribute::EventListener(name, listener.map_callback(cb))
            }
            Attribute::Plain(plain) => Attribute::Plain(plain),
            Attribute::Style(styles) => Attribute::Style(styles),
            Attribute::FunctionCall(fn_name, func_call) => {
                Attribute::FunctionCall(fn_name, func_call)
            }
        }
    }

    /// transform the callback of this attribute where EVENT becomes EVENT2
    pub fn reform<F, EVENT2>(self, func: F) -> Attribute<ATT, EVENT2, MSG>
    where
        F: Fn(EVENT2) -> EVENT + 'static,
        EVENT2: 'static,
    {
        match self {
            Attribute::EventListener(event_name, cb) => {
                Attribute::EventListener(event_name, cb.reform(func))
            }
            Attribute::Plain(plain) => Attribute::Plain(plain),
            Attribute::Style(styles) => Attribute::Style(styles),
            Attribute::FunctionCall(fn_name, func_call) => {
                Attribute::FunctionCall(fn_name, func_call)
            }
        }
    }
}

impl<ATT, EVENT, MSG> fmt::Display for Attribute<ATT, EVENT, MSG>
where
    ATT: ToString + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.render(f)
    }
}
