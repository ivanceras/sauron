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
