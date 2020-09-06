pub struct Theme<'a> {
    pub hover_color: &'a str,
    pub hover_shadow: &'a str,
    pub border_color: &'a str,
    pub corner_color: &'a str,
    pub border_shadow: &'a str,
    pub corner_shadow: &'a str,
    pub content_background_color: &'a str,
    pub button_text_color: &'a str,
    pub highlight_color: &'a str,
}

pub struct Colors<'a> {
    pub primary: &'a str,
    pub secondary: &'a str,
    pub header: &'a str,
    pub control: &'a str,
    pub success: &'a str,
    pub alert: &'a str,
    pub disabled: &'a str,
}

impl<'a> Theme<'a> {
    // base theme
    pub fn base() -> Self {
        Theme {
            hover_color: "#26dafd",
            hover_shadow: "#029dbb",
            border_color: "#029dbb",
            corner_color: "#26dafd",
            border_shadow: "rgba(2,157,187,0.65)",
            corner_shadow: "rgba(38,218,253,0.65)",
            content_background_color: "rgba(4,35,41,0.65)",
            button_text_color: "#acf9fb",
            highlight_color: "#029dbb",
        }
    }

    pub fn alt() -> Self {
        Theme {
            hover_color: "green",
            hover_shadow: "yellow",
            border_color: "#090",
            corner_color: "#0f0",
            button_text_color: "#0f0",
            border_shadow: "rgba(0,153,0,0.65)",
            corner_shadow: "rgba(0,255,0,0.65)",
            content_background_color: "rgba(10,50,10,0.65)",
            highlight_color: "#090",
        }
    }

    pub fn disabled() -> Self {
        Theme {
            hover_color: "#fff",
            hover_shadow: "#eee",
            border_color: "#666",
            corner_color: "#999",
            corner_shadow: "rgba(153,153,153,0.65)",
            border_shadow: "rgba(102,102,102,0.65)",
            button_text_color: "#999",
            content_background_color: "rgba(20,20,20,0.65)",
            highlight_color: "#fff",
        }
    }
}

impl<'a> Colors<'a> {
    fn color() -> Self {
        Colors {
            primary: "#26dafd",
            secondary: "#df9527",
            header: "#a1ecfb",
            control: "#acf9fb",
            success: "#00ff00",
            alert: "#ff0000",
            disabled: "#999999",
        }
    }

    fn background() -> Self {
        Colors {
            primary: "#021114",
            secondary: "#180f02",
            header: "#032026",
            control: "#041e1f",
            success: "#081402",
            alert: "#140202",
            disabled: "#141414",
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
    fn test_generate_from_primary() {
        let hex = "#26dafd";
        let base = hex_to_real_rgba(hex);
        println!("base: {}", base);

        assert_eq!(base, rgba(38, 218, 253, 1.0));

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

        let lighter = base.lighten(percent(20));
        println!("lighter: {}", lighter);
        assert_eq!(lighter, rgba(103, 255, 103, 1.00));

        let dark = base.darken(percent(20));
        println!("dark: {}", dark);
        assert_eq!(dark, rgba(0, 154, 0, 1.00));
    }
}
