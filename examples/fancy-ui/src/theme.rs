use css_color::ParseColorError;
use css_colors::{percent, rgba, Color, RGBA};
use sauron::jss;

#[derive(PartialEq, Debug, Clone)]
pub struct Theme {
    pub primary_color: String,    // used in container
    pub secondary_color: String,  // used in container
    pub background_color: String, // used in container
    pub accent_color: String,
    pub accent_shadow: String,
    status_pallete: StatusPallete,
    pub controls: Controls,
}

/// Issue how to derive pallet from primary?
/// Maybe mix the pallet color with the primary color
#[derive(PartialEq, Debug, Clone)]
pub struct StatusPallete {
    /// color for error, default is red
    pub error: RGBA,
    /// color for success, default is green
    pub success: RGBA,
    /// color for info, default is blue
    pub info: RGBA,
    /// color for warning, default is yellow
    pub warning: RGBA,
}

impl Default for StatusPallete {
    fn default() -> Self {
        Self {
            error: rgba(255, 0, 0, 1.0),
            success: rgba(0, 255, 0, 1.0),
            info: rgba(0, 0, 255, 1.0),
            warning: rgba(255, 255, 0, 1.0),
        }
    }
}

/// colors to controls
/// such as buttons, navigation links, frames
#[derive(PartialEq, Debug, Clone)]
pub struct Controls {
    pub hover_color: String,
    pub hover_shadow: String,
    pub border_color: String,
    pub corner_color: String,
    pub border_shadow: String,
    pub corner_shadow: String,
    // used a background in frames and buttons
    pub content_background_color: String,
    // text color in buttons
    pub button_text_color: String,
    // highlight color in buttons
    pub highlight_color: String,
    // text color in links
    pub link_color: String,
}

impl Theme {
    /// create a them from color that can be parse
    #[allow(unused)]
    pub fn from_str(primary: &str, background: &str) -> Result<Self, ParseColorError> {
        let primary = hex_to_real_rgba(primary);
        let background = hex_to_real_rgba(background);
        Ok(Self::calculate_theme(
            primary?,
            background?,
            StatusPallete::default(),
        ))
    }

    pub fn style(&self) -> Vec<String> {
        vec![jss! {
            "body" : {
                background_color: self.background_color.clone(),
                color: self.primary_color.clone(),
            }
        }]
    }

    pub fn primary_color(&self) -> String {
        self.primary_color.clone()
    }

    pub fn background_color(&self) -> String {
        self.background_color.clone()
    }

    // base theme using a bluish base color #029dbb
    #[allow(unused)]
    pub fn bondi_blue_on_dark() -> Self {
        let primary = rgba(2, 157, 187, 1.0); // main theme
        let background = rgba(0, 0, 0, 1.0);
        Self::calculate_theme(primary, background, StatusPallete::default())
    }

    #[allow(unused)]
    pub fn white_on_dark() -> Self {
        let primary = rgba(255, 255, 255, 1.0);
        let background = rgba(0, 0, 0, 1.0);
        Self::calculate_theme(primary, background, StatusPallete::default())
    }

    #[allow(unused)]
    pub fn green_on_black() -> Self {
        let primary = rgba(0, 255, 0, 1.0);
        let background = rgba(0, 0, 0, 1.0);
        Self::calculate_theme(primary, background, StatusPallete::default())
    }

    #[allow(unused)]
    pub fn black_on_white() -> Self {
        Self::calculate_theme(
            rgba(0, 0, 0, 1.0),
            rgba(255, 255, 255, 1.0),
            StatusPallete::default(),
        )
    }

    /// light: if background is light and foreground is dark
    pub fn calculate_theme(
        foreground: RGBA,
        background: RGBA,
        status_pallete: StatusPallete,
    ) -> Self {
        let grey = rgba(128, 128, 128, 1.0);
        let light = background.is_lighter(&grey);

        let primary = foreground;
        let accent = if light {
            primary.shade(percent(30))
        } else {
            primary.tint(percent(30))
        };

        let secondary = if light {
            primary.darken(percent(20))
        } else {
            primary.lighten(percent(20))
        };

        let text_colors = if light {
            primary.darken(percent(40))
        } else {
            primary.lighten(percent(40))
        };

        let background_color = if light {
            background.lighten(percent(60))
        } else {
            primary.darken(percent(60))
        };

        let accent_shadow = if light {
            accent.fadeout(percent(35))
        } else {
            accent.fadein(percent(35))
        };

        let corner_shadow = if light {
            secondary.fadein(percent(35))
        } else {
            secondary.fadeout(percent(35))
        };

        let content_background_color = if light {
            primary.mix(background, percent(15)).fadein(percent(35))
        } else {
            primary.mix(background, percent(15)).fadeout(percent(35))
        };

        let status_pallete = if light {
            let error = status_pallete
                .error
                .mix(background, percent(80))
                .fadein(percent(20));
            let success = status_pallete
                .success
                .mix(background, percent(80))
                .fadein(percent(20));
            let info = status_pallete
                .info
                .mix(background, percent(80))
                .fadein(percent(20));
            let warning = status_pallete
                .warning
                .mix(background, percent(80))
                .fadein(percent(20));
            StatusPallete {
                error,
                success,
                info,
                warning,
            }
        } else {
            let error = status_pallete
                .error
                .mix(background, percent(80))
                .fadeout(percent(20));
            let success = status_pallete
                .success
                .mix(background, percent(80))
                .fadeout(percent(20));
            let info = status_pallete
                .info
                .mix(background, percent(80))
                .fadeout(percent(20));
            let warning = status_pallete
                .warning
                .mix(background, percent(80))
                .fadeout(percent(20));
            StatusPallete {
                error,
                success,
                info,
                warning,
            }
        };

        Theme {
            primary_color: primary.to_css(),
            secondary_color: secondary.to_css(),
            background_color: background_color.to_css(),
            accent_color: accent.to_css(),
            accent_shadow: accent_shadow.to_css(),
            status_pallete,

            controls: Controls {
                hover_shadow: primary.to_css(),
                border_color: primary.to_css(),
                border_shadow: primary.to_css(),
                highlight_color: primary.to_css(),

                hover_color: secondary.to_css(),
                corner_color: secondary.to_css(),
                corner_shadow: corner_shadow.to_css(),
                content_background_color: content_background_color.to_css(),
                button_text_color: text_colors.to_css(),
                link_color: accent.to_css(),
            },
        }
    }
}

impl Theme {
    pub fn error(&self) -> RGBA {
        self.status_pallete.error
    }

    pub fn warning(&self) -> RGBA {
        self.status_pallete.warning
    }

    pub fn info(&self) -> RGBA {
        self.status_pallete.info
    }

    pub fn success(&self) -> RGBA {
        self.status_pallete.success
    }
}

impl Default for Theme {
    fn default() -> Self {
        // Self::black_on_white()
        //Self::white_on_dark()
        //Self::green_on_black()
        Self::bondi_blue_on_dark()
    }
}

/// convert from color to colors version
fn convert_to_real_rgba(color: css_color::Rgba) -> RGBA {
    let red = (color.red * 255.0) as u8;
    let green = (color.green * 255.0) as u8;
    let blue = (color.blue * 255.0) as u8;
    rgba(red, green, blue, color.alpha)
}

fn hex_to_real_rgba(hex: &str) -> Result<RGBA, ParseColorError> {
    let from_hex: css_color::Rgba = hex.parse()?;
    Ok(convert_to_real_rgba(from_hex))
}

trait IsLighter {
    fn is_lighter(&self, other: &Self) -> bool;
}

impl<T> IsLighter for T
where
    T: Color + Clone,
{
    fn is_lighter(&self, other: &Self) -> bool {
        let this = self.clone().to_rgb().greyscale();
        dbg!(&this);
        assert_eq!(this.r, this.g);
        assert_eq!(this.g, this.b);
        let other = other.clone().to_rgb().greyscale();
        dbg!(&other);
        assert_eq!(other.r, other.g);
        assert_eq!(other.g, other.b);

        this.r > other.r
    }
}

#[cfg(test)]
mod tests;
