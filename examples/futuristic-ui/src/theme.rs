use css_colors::{
    percent,
    rgba,
    Color,
};

pub struct Theme {
    pub primary_color: String,    // used in container
    pub secondary_color: String,  // used in container
    pub background_color: String, // used in container
    pub accent_color: String,
    pub accent_shadow: String,
    pub primary_font: String,
    pub secondary_font: String,
    pub controls: Controls,
}

/// colors to controls
/// such as buttons, navigation links, frames
pub struct Controls {
    pub hover_color: String,
    pub hover_shadow: String,
    pub border_color: String,
    pub corner_color: String,
    pub border_shadow: String,
    pub corner_shadow: String,
    pub content_background_color: String,
    pub button_text_color: String,
    pub highlight_color: String,
}

impl Theme {
    // base theme using a bluish base color #029dbb
    pub fn base() -> Self {
        let primary = rgba(2, 157, 187, 1.0);
        //let primary = rgba(255, 0, 255, 1.0);

        let accent = primary.tint(percent(30)); //combine with 30% of white becomes rgba(179, 225, 234, 1.00)
                                                //target color is rgba(161, 236, 251, 1.00);
        let secondary = primary.lighten(percent(20));
        let text_colors = primary.lighten(percent(40));
        let primary_font = "\"Titillium Web\", \"sans-serif\"".to_string();
        let secondary_font = "\"Electrolize\", \"sans-serif\"".to_string();
        Theme {
            primary_color: primary.to_css(),
            secondary_color: secondary.to_css(),
            background_color: primary.darken(percent(60)).to_css(),
            accent_color: accent.to_css(),
            accent_shadow: accent.fadein(percent(35)).to_css(),
            primary_font,
            secondary_font,

            controls: Controls {
                hover_shadow: primary.to_css(),
                border_color: primary.to_css(),
                border_shadow: primary.to_css(),
                highlight_color: primary.to_css(),

                hover_color: secondary.to_css(),
                corner_color: secondary.to_css(),
                corner_shadow: secondary.fadeout(percent(35)).to_css(),

                content_background_color: primary
                    .darken(percent(30))
                    .fadeout(percent(35))
                    .to_css(),
                button_text_color: text_colors.to_css(),
            },
        }
    }

    // alternate color for the button
    pub fn alt() -> Self {
        let primary = rgba(0, 255, 0, 1.0); //#00ff00
        let secondary = primary.lighten(percent(20));
        let text_colors = primary.lighten(percent(40));
        let accent = rgba(0, 100, 0, 1.00);
        let primary_font = "\"Titillium Web\", \"sans-serif\"".to_string();
        let secondary_font = "\"Electrolize\", \"sans-serif\"".to_string();
        Theme {
            primary_color: primary.to_css(),
            secondary_color: secondary.to_css(),
            background_color: primary.darken(percent(60)).to_css(),
            accent_color: accent.to_css(),
            accent_shadow: accent.fadein(percent(35)).to_css(),
            primary_font,
            secondary_font,

            controls: Controls {
                hover_shadow: primary.to_css(),
                border_color: primary.to_css(),
                border_shadow: primary.to_css(),
                highlight_color: primary.to_css(),

                hover_color: secondary.to_css(),
                corner_color: secondary.to_css(),
                corner_shadow: secondary.fadeout(percent(35)).to_css(),

                content_background_color: primary
                    .darken(percent(40))
                    .fadeout(percent(35))
                    .to_css(),
                button_text_color: text_colors.to_css(),
            },
        }
    }

    // color for the disabled button
    pub fn disabled() -> Self {
        let primary = rgba(255, 255, 255, 1.0); // #ffffff
        let secondary = primary.lighten(percent(20));
        let text_colors = primary.lighten(percent(30));
        let accent = rgba(100, 100, 100, 1.00);
        let primary_font = "\"Titillium Web\", \"sans-serif\"".to_string();
        let secondary_font = "\"Electrolize\", \"sans-serif\"".to_string();

        Theme {
            primary_color: primary.to_css(),
            secondary_color: secondary.to_css(),
            background_color: primary.darken(percent(60)).to_css(),
            accent_color: accent.to_css(),
            accent_shadow: accent.fadein(percent(35)).to_css(),
            primary_font,
            secondary_font,

            controls: Controls {
                hover_shadow: primary.to_css(),
                border_color: primary.to_css(),
                border_shadow: primary.to_css(),
                highlight_color: primary.to_css(),

                hover_color: secondary.to_css(),
                corner_color: secondary.to_css(),
                corner_shadow: secondary.fadeout(percent(35)).to_css(),

                content_background_color: primary
                    .darken(percent(80))
                    .fadeout(percent(35))
                    .to_css(),
                button_text_color: text_colors.to_css(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use css_color::Rgba;
    use css_colors::{
        percent,
        rgb,
        rgba,
        Color,
        RGBA,
    };

    /// convert from color to colors version
    fn convert_to_real_rgba(color: Rgba) -> RGBA {
        let red = (color.red * 255.0) as u8;
        let green = (color.green * 255.0) as u8;
        let blue = (color.blue * 255.0) as u8;
        rgba(red, green, blue, color.alpha)
    }

    fn hex_to_real_rgba(hex: &str) -> RGBA {
        let from_hex: Rgba = hex.parse().expect("must parse");
        convert_to_real_rgba(from_hex)
    }

    #[test]
    fn test_colors() {
        let salmon = rgb(250, 128, 114);

        assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    }

    #[test]
    fn equivalent_colors() {
        let hex = "#029dbb"; //border_color,
        let c = hex_to_real_rgba(hex);
        println!("c:{}", c);
        assert_eq!(c, rgba(2, 157, 187, 1.0));
        let light_c = c.lighten(percent(20)); // hover_color, corner_color,
        assert_eq!(light_c, rgba(39, 217, 253, 1.0));

        let light_c2 = c.lighten(percent(40));
        assert_eq!(light_c2, rgba(140, 235, 254, 1.0));

        let dark_c = c.darken(percent(20));
        println!("dark c: {}", dark_c);
        assert_eq!(dark_c, rgba(1, 73, 87, 1.0));

        let button_text_color = "#acf9fb";
        let button_text_color = hex_to_real_rgba(button_text_color);
        println!("button_text_color: {}", button_text_color);
        assert_eq!(button_text_color, rgba(172, 249, 251, 1.0));

        assert_eq!(
            c.darken(percent(20)).fadeout(percent(35)),
            rgba(1, 73, 87, 0.65)
        );

        println!("tint1: {}", c.tint(percent(10)));
        println!("tint2: {}", c.tint(percent(20)));
        println!("tint3: {}", c.tint(percent(30)));
        println!("tint5: {}", c.tint(percent(50)));
        assert_eq!(rgba(204, 235, 241, 1.0), c.tint(percent(20)));
    }

    #[test]
    fn test_generate_from_secondary() {
        let hex = "#26dafd"; // very close to #029dbb -> lighten 20% = rgba(39, 217, 253, 1.0) which is the secondary color
        let base = hex_to_real_rgba(hex);
        println!("base: {}", base);

        assert_eq!(base, rgba(38, 218, 253, 1.0));

        let grey = base.greyscale();
        println!("grey: {}", grey);
        assert_eq!(grey, rgba(146, 146, 146, 1.0));

        let lighter = base.lighten(percent(20));
        println!("lighter: {}", lighter);
        assert_eq!(lighter, rgba(140, 235, 254, 1.00));

        let dark = base.darken(percent(20));
        println!("dark: {}", dark);
        assert_eq!(dark, rgba(2, 157, 188, 1.00));

        let fade_out = base.fadeout(percent(65));
        println!("fade_out: {}", fade_out);
        assert_eq!(fade_out, rgba(38, 218, 253, 0.35));

        // seems like the logic to change alpha is just 100%-arg%
        let fade_out2 = base.fadeout(percent(35));
        println!("fade_out2: {}", fade_out2);
        assert_eq!(fade_out2, rgba(38, 218, 253, 0.65));

        // can not fade in more when alpha is already 1.0
        let fade_in2 = base.fadein(percent(35));
        println!("fade_in2: {}", fade_in2);
        assert_eq!(fade_in2, rgba(38, 218, 253, 1.0));

        let lighter_fadeout = lighter.fadeout(percent(35));
        println!("lighter_fadeout: {}", lighter_fadeout);
        assert_eq!(lighter_fadeout, rgba(140, 235, 254, 0.65));
    }

    #[test]
    fn test_generate_from_header() {
        let hex = "#a1ecfb";
        let base = hex_to_real_rgba(hex);
        println!("base: {}", base);

        assert_eq!(base, rgba(161, 236, 251, 1.00));

        let lighter = base.lighten(percent(20));
        println!("lighter: {}", lighter);
        assert_eq!(lighter, rgba(255, 255, 255, 1.00));

        let dark = base.darken(percent(20));
        println!("dark: {}", dark);
        assert_eq!(dark, rgba(63, 216, 247, 1.00));
    }

    #[test]
    fn test_generate_from_alt_color() {
        let hex = "#00ff00";
        let base = hex_to_real_rgba(hex);
        println!("base: {}", base);

        assert_eq!(base, rgba(0, 255, 0, 1.0));

        let grey = base.greyscale();
        println!("grey: {}", grey);
        assert_eq!(grey, rgba(128, 128, 128, 1.0));

        let lighter = base.lighten(percent(20));
        println!("lighter: {}", lighter);
        assert_eq!(lighter, rgba(103, 255, 103, 1.00));

        let dark = base.darken(percent(20));
        println!("dark: {}", dark);
        assert_eq!(dark, rgba(0, 154, 0, 1.00));
    }

    #[test]
    fn test_color_processing() {
        use color_processing::Color;
        let hex = "#26dafd";
        let base = Color::new_string(hex).unwrap();

        println!("base: {}", base.to_rgb_string());
        assert_eq!(base.to_rgb_string(), "rgb(38, 218, 253)");

        let brighten = base.brighten(0.2);
        println!("brighten: {}", brighten.to_rgb_string());
        assert_eq!(brighten.to_rgb_string(), "rgb(59, 228, 255)");

        let darken = base.darken(0.2);
        println!("darken: {}", darken.to_rgb_string());
        assert_eq!(darken.to_rgb_string(), "rgb(0, 208, 243)");
    }
}
