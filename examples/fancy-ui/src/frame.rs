use crate::Status;
use crate::Theme;
use css_colors::Color;
use sauron::dom::DomNode;
use sauron::{
    dom::WebComponent,
    html::{attributes::*, events::*, *},
    vdom::Callback,
    *,
};
use std::collections::BTreeMap;
use std::str::FromStr;
use web_sys::MouseEvent;

const COMPONENT_NAME: &str = "sfui-frame";
const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 40;

#[derive(Clone, Debug)]
pub enum Msg<XMSG> {
    Click(MouseEvent),
    HoverIn,
    HoverOut,
    HighlightEnd,
    External(XMSG),
    ContentTargetMounted(MountEvent),
}

#[derive(Debug)]
pub struct Frame<XMSG> {
    feature: Feature,
    clicked: bool,
    hovered: bool,
    click_listeners: Vec<Callback<MouseEvent, XMSG>>,
    /// called when the container for the content is mounted
    container_mounted_listeners: Vec<Callback<MountEvent, XMSG>>,
    width: Option<usize>,
    height: Option<usize>,
    theme: Theme,
    /// the status of the button which changes the color pallet of the button
    status: Option<Status>,
    children: Vec<DomNode>,
    content_target_node: Option<DomNode>,
    dimension: Dimension,
}

#[derive(Debug)]
pub struct Feature {
    /// has corners
    pub has_corners: bool,
    /// corner lies outward of the border lines
    /// if false, corner lines aligns to the border lines
    pub outward_corners: bool,
    /// the frame has borders
    pub has_borders: bool,
    /// wheter the border extend all the way through the full size of the frame
    /// if false, the border will have stop at some distance from the corner of the frame
    pub full_borders: bool,
    /// expand corners when hovered
    pub expand_corners: bool,
    pub has_corner_box_shadow: bool,
    pub has_border_box_shadow: bool,
}

#[derive(Debug, Clone)]
pub struct Dimension {
    /// width of the corner clip of this button
    pub corner_width: i32,
    /// lengths of the corner clip of this button
    pub corner_length: i32,
    /// distance that clips at the corner expands when the button is hovered
    pub corner_expand_distance: i32,
    /// the padding for the content of the frame
    pub content_padding: i32,
}

impl Default for Dimension {
    fn default() -> Self {
        Self::large()
    }
}

impl Dimension {
    fn large() -> Self {
        Self {
            corner_width: 4,
            corner_length: 16,
            corner_expand_distance: 12,
            content_padding: 0,
        }
    }
    pub fn small() -> Self {
        Self {
            corner_width: 2,
            corner_length: 8,
            corner_expand_distance: 6,
            content_padding: 0,
        }
    }

    pub fn static_frame() -> Self {
        Self {
            corner_width: 2,
            corner_length: 8,
            corner_expand_distance: 6,
            content_padding: 10,
        }
    }
}

impl<XMSG> Default for Frame<XMSG> {
    fn default() -> Self {
        Self {
            feature: Feature::default(),
            clicked: false,
            hovered: false,
            click_listeners: vec![],
            container_mounted_listeners: vec![],
            width: None,
            height: None,
            theme: Theme::default(),
            status: None,
            children: vec![],
            content_target_node: None,
            dimension: Dimension::default(),
        }
    }
}

impl<XMSG> Frame<XMSG> {
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn set_feature(&mut self, feature: Feature) {
        self.feature = feature;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = Some(status);
    }
    fn computed_width(&self) -> usize {
        // use the supplied width if it is specified
        if let Some(width) = self.width {
            width
        } else {
            DEFAULT_WIDTH
        }
    }

    pub fn set_dimension(&mut self, dimension: Dimension) {
        self.dimension = dimension;
    }

    fn computed_height(&self) -> usize {
        if let Some(height) = self.height {
            height
        } else {
            DEFAULT_HEIGHT
        }
    }

    fn view_borders(&self) -> Node<Msg<XMSG>> {
        node_list([
            view_if(
                self.feature.has_borders,
                div([class("border border-left")], []),
            ),
            view_if(
                self.feature.has_borders,
                div([class("border border-right")], []),
            ),
            view_if(
                self.feature.has_borders,
                div([class("border border-top")], []),
            ),
            view_if(
                self.feature.has_borders,
                div([class("border border-bottom")], []),
            ),
        ])
    }

    fn view_corners(&self) -> Node<Msg<XMSG>> {
        node_list([
            view_if(
                self.feature.has_corners,
                div([class("corner corner__top-left")], []),
            ),
            view_if(
                self.feature.has_corners,
                div([class("corner corner__bottom-left")], []),
            ),
            view_if(
                self.feature.has_corners,
                div([class("corner corner__top-right")], []),
            ),
            view_if(
                self.feature.has_corners,
                div([class("corner corner__bottom-right")], []),
            ),
        ])
    }
}

impl<XMSG> Component for Frame<XMSG>
where
    XMSG: 'static,
{
    type MSG = Msg<XMSG>;
    type XMSG = XMSG;

    fn init(&mut self) -> Effects<Msg<XMSG>, XMSG> {
        Effects::none()
    }

    fn update(&mut self, msg: Msg<XMSG>) -> Effects<Msg<XMSG>, XMSG> {
        match msg {
            Msg::Click(mouse_event) => {
                self.clicked = true;
                let pmsg_list = self
                    .click_listeners
                    .iter()
                    .map(|listener| listener.emit(mouse_event.clone()));
                Effects::with_external(pmsg_list)
            }
            Msg::HoverIn => {
                self.hovered = true;
                Effects::none()
            }
            Msg::HoverOut => {
                self.hovered = false;
                Effects::none()
            }
            Msg::HighlightEnd => {
                self.clicked = false;
                Effects::none()
            }
            Msg::External(xmsg) => Effects::with_external([xmsg]),
            Msg::ContentTargetMounted(me) => {
                log::info!("frame child container is now mounted...");
                let mount_event = me.clone();
                let target_node = me.target_node;
                target_node.append_children(self.children.drain(..).collect());
                self.content_target_node = Some(target_node);
                let external = self
                    .container_mounted_listeners
                    .iter()
                    .map(|listener| listener.emit(mount_event.clone()));
                Effects::with_external(external)
            }
        }
    }

    fn view(&self) -> Node<Msg<XMSG>> {
        div(
            [
                class(COMPONENT_NAME),
                classes_flag([
                    ("clicked", self.clicked),
                    ("expand_corners", self.feature.expand_corners),
                    ("has_corner_box_shadow", self.feature.has_corner_box_shadow),
                    ("has_border_box_shadow", self.feature.has_border_box_shadow),
                    ("hovered", self.hovered),
                ]),
                if let Some(ref status) = self.status {
                    class(status.class_name())
                } else {
                    empty_attr()
                },
                // normally click should be attached to the actual button element
                on_click(Msg::Click),
                // the mouseover events are attached here since the hover element z-index is
                // higher than the actual button, which will cause a janky animation
                // when the mouse is triggering alt hover in and out, since covered by the hover
                // layer effect
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            [div(
                [],
                [
                    // borders
                    self.view_borders(),
                    // corners
                    self.view_corners(),
                    div(
                        [
                            class("content_wrap"),
                            on_mount(|me| Msg::ContentTargetMounted(me)),
                        ],
                        [],
                    ),
                ],
            )],
        )
    }

    fn stylesheet() -> Vec<String> {
        let theme = Theme::default();
        let base = &theme.controls;
        let transition_time_ms = Self::transition_time_ms(); //transition time for most effects on the button
        let hover_transition_time = 100; // the transition of the lower highligh of the button when hovering
        let highlight_transition = 50; // the transition time for the highlight color of the button when clicked

        //let width = self.computed_width();
        //let height = self.computed_height();
        let dimension = Dimension::default();
        let content_padding = dimension.content_padding;

        let main = jss! {
            // the ROOT component style
            format!(".{}",COMPONENT_NAME): {
                display: "inline-block",
                padding: px(1),
                position: "relative",
                margin: px([10, 10]),
            },

            ".content_wrap": {
                padding: px(content_padding),
            },

            ".hidden" : {
                visibility: "hidden",
            },
        };

        vec![main, Self::border_style(), Self::corner_style()]
    }

    /// what attributes this component is interested in
    fn observed_attributes() -> Vec<&'static str> {
        vec!["theme-primary", "theme-background", "feature", "status"]
    }

}

impl<XMSG> Frame<XMSG>
where
    XMSG: 'static,
{
    pub fn with_options(mut self, feature: Feature) -> Self {
        self.feature = feature;
        self
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn add_click_listener<F>(&mut self, f: F)
    where
        F: Fn(MouseEvent) -> XMSG + 'static,
    {
        let cb = Callback::from(f);
        self.click_listeners.push(cb);
    }

    pub fn container_mounted_listener<F>(&mut self, f: F)
    where
        F: Fn(MountEvent) -> XMSG + 'static,
    {
        let cb = Callback::from(f);
        self.container_mounted_listeners.push(cb);
    }

    ///transition time for most effects on the frame
    fn transition_time_ms() -> usize {
        250
    }

    fn border_style() -> String {
        let theme = Theme::default();
        let border_width = 1; // the width of the border for each side of the button
        let base = &theme.controls;
        let transition_time_ms = Self::transition_time_ms(); //transition time for most effects on the button
                                                             //
        let Dimension { corner_length, .. } = Dimension::default();

        let feature = Feature::default();

        let border_cut = if feature.full_borders {
            0
        } else {
            (corner_length + 4) * 2
        };

        jss! {
            // BORDERS these are styled divs wrapping the buttons
            ".border": {
                border_color: base.border_color.clone(),
                z_index: 1,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in",transition_time_ms),
                border_style: "solid",
            },

            ".has_border_box_shadow .border":{
                box_shadow: format!("{} {}",px([0,0,4]), base.border_shadow.clone()),
            }

            ".error .border": {
                border_color: theme.error().to_css(),
                box_shadow: format!("{} {}",px([0, 0, 4]), theme.error().to_css()),
            },

            ".success .border": {
                border_color: theme.success().to_css(),
                box_shadow: format!("{} {}",px([0, 0, 4]), theme.success().to_css()),
            },

            ".info .border": {
                border_color: theme.info().to_css(),
                box_shadow: format!("{} {}",px([0, 0, 4]), theme.info().to_css()),
            },

            ".warning .border": {
                border_color: theme.warning().to_css(),
                box_shadow: format!("{} {}",px([0, 0, 4]), theme.warning().to_css()),
            },


            ".border-left": {
                top: percent(50),
                left: 0,
                height: format!("calc({} - {})", percent(100), px(border_cut)),
                transform: format!("translate({}, {})", 0, percent(-50)),
                border_width: px([0, 0, 0, border_width]),
            },

            ".border-right": {
                top: percent(50),
                right: 0,
                height: format!("calc({} - {})", percent(100), px(border_cut)),
                transform: format!("translate({}, {})", 0, percent(-50)),
                border_width: px([0, 0, 0, border_width]),
            },

            ".border-top": {
                top: 0,
                left: percent(50),
                width: format!("calc({} - {})", percent(100), px(border_cut)),
                transform: format!("translate({}, {})", percent(-50), 0),
                border_width: px([border_width, 0, 0, 0]),
            },

            ".border-bottom": {
                left: percent(50),
                width: format!("calc({} - {})", percent(100),  px(border_cut)),
                bottom: 0,
                transform: format!("translate({}, {})", percent(-50), 0),
                border_width: px([border_width, 0, 0, 0]),
            },
        }
    }

    fn corner_style() -> String {
        let theme = Theme::default();
        let base = &theme.controls;
        let transition_time_ms = Self::transition_time_ms();
        let feature = Feature::default();
        let outward_corners = feature.outward_corners;

        let Dimension {
            corner_width,
            corner_length,
            corner_expand_distance,
            ..
        } = Dimension::default();

        // arm of top, left bottom, right for the corners
        let arm = if outward_corners {
            -corner_width
        } else {
            -corner_width / 2
        };

        jss! {
            // CORNERS - the fancy divs which clips the button
            ".corner": {
                width: px(corner_length),
                height: px(corner_length),
                border_color: base.corner_color.clone(),
                z_index: 2,
                opacity: 1,
                position: "absolute",
                transition: format!("all {}ms ease-in",transition_time_ms),
                border_style: "solid",
            },

            ".has_corner_box_shadow .corner": {
                box_shadow: format!("{} {}",px([0, 0, 4]), base.corner_shadow.clone()),
            },

            ".error .corner": {
                border_color: theme.error().to_css(),
            },

            ".success  .corner": {
                border_color: theme.success().to_css(),
            },

            ".info  .corner": {
                border_color: theme.info().to_css(),
            },

            ".warning  .corner": {
                border_color: theme.warning().to_css(),
            },


            ".corner__top-left": {
                left: px(arm),
                top: px(arm),
                border_width: px([corner_width, 0, 0, corner_width]),
            },

            ".corner__bottom-left": {
                left: px(arm),
                bottom: px(arm),
                border_width: px([0, 0, corner_width, corner_width]),
            },

            ".corner__top-right": {
                right: px(arm),
                top: px(arm),
                border_width: px([corner_width, corner_width, 0, 0]),
            },

            ".corner__bottom-right": {
                right: px(arm),
                bottom: px(arm),
                border_width: px([0, corner_width, corner_width, 0]),
            },

            // if expand_corners is enabled
            // the fui_button corners will EXPAND when hovered.
            //
            // CSS Notes:
            // - `.class1.class2 child` means if both class1 and class2 is specified in the
            // parent, the properties will be applied to this child element
            //
            //  - `.class1,.class2 child` means either if either class1 or class2 is specified in the
            // parent, the properties will be applied to this child element
            //
            ".expand_corners.hovered  .corner__top-left": {
                left: px(-corner_expand_distance),
                top: px(-corner_expand_distance),
            },

            ".expand_corners.hovered  .corner__bottom-left": {
                left: px(-corner_expand_distance),
                bottom: px(-corner_expand_distance),
            },

            ".expand_corners.hovered  .corner__top-right": {
                right: px(-corner_expand_distance),
                top: px(-corner_expand_distance),
            },

            ".expand_corners.hovered .corner__bottom-right": {
                right: px(-corner_expand_distance),
                bottom: px(-corner_expand_distance),
            },
        }
    }
}

impl Default for Feature {
    fn default() -> Self {
        Self::tight()
    }
}

impl Feature {
    fn tight() -> Self {
        Self {
            has_corners: true,
            outward_corners: true,
            has_borders: true,
            full_borders: true,
            expand_corners: true,
            has_corner_box_shadow: true,
            has_border_box_shadow: true,
        }
    }
    fn static_frame() -> Self {
        Self {
            has_corners: true,
            outward_corners: false,
            has_borders: true,
            full_borders: false,
            expand_corners: false,
            has_corner_box_shadow: false,
            has_border_box_shadow: false,
        }
    }
}

impl<XMSG> StatefulComponent for Frame<XMSG> {
    /// called when any of the attributes in observed_attributes is changed
    fn attribute_changed(
        &mut self,
        attr_name: &str,
        _old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) {
        match attr_name {
            "theme-primary" => {
                if let Some(primary) = new_value.get_string() {
                    //let background = &app.theme.background_color;
                    //app.theme =
                    //    Theme::from_str(&primary, background).expect("must be a valid theme");
                }
            }
            "theme-background" => {
                if let Some(background) = new_value.get_string() {
                    //let primary = &app.theme.primary_color;
                    //app.theme =
                    //    Theme::from_str(primary, &background).expect("must be a valid theme");
                }
            }
            "status" => {
                if let Some(v) = new_value.get_string() {
                    //app.status = Status::from_str(&v).ok();
                }
            }
            _ => (),
        }
    }

    fn child_container(&self) -> Option<DomNode>{
        if let Some(content_target_node) = self.content_target_node.as_ref(){
            Some(content_target_node.clone())
        }else{
            log::warn!("There is no content target node...");
            None
        }
    }

    fn append_children(&mut self, children: Vec<DomNode>) {
        self.children.extend(children);
    }

    fn connected_callback(&mut self) {}
    fn disconnected_callback(&mut self) {}
    fn adopted_callback(&mut self) {}
}
