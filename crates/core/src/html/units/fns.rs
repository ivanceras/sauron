use std::fmt::Display;

/// the [rgb](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/rgb) css function
pub fn rgb(r: impl Display, g: impl Display, b: impl Display) -> String {
    format!("rgb({r}, {g}, {b})")
}

/// the [rgba](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/rgb) css function
pub fn rgba(r: impl Display, g: impl Display, b: impl Display, a: impl Display) -> String {
    format!("rgba({r}, {g}, {b}, {a})")
}

/// rotate function
/// `rotate(deg(360))`
pub fn rotate(a: impl Display) -> String {
    format!("rotate({a})")
}
