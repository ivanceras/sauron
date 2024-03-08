
use sauron::wasm_bindgen::JsCast;
use sauron::{
    custom_element, vdom::Callback, html::attributes::*, html::events::*, html::*, jss,
    wasm_bindgen, web_sys, Attribute, Effects, JsValue, Node, WebComponent, *,
};
use std::fmt::Debug;
use sauron::dom::DomAttrValue;
use sauron::vdom::AttributeName;
use sauron::dom::DomAttr;
use sauron::dom::StatefulComponent;
use sauron::dom::template;
use sauron::dom::DomNode;
use std::default::Default;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    IntervalChange(f64),
    Mounted(MountEvent),
    ExternContMounted(web_sys::Node),
    BtnClick,
}

#[derive(Debug, Clone)]
pub struct DateTimeWidget {
    /// the host element the web editor is mounted to, when mounted as a custom web component
    host_element: Option<web_sys::Element>,
    children: Vec<web_sys::Node>,
    external_children_node: Option<web_sys::Node>,
    date: String,
    time: String,
    cnt: i32,
}

impl Default for DateTimeWidget {
    fn default() -> Self {
        Self {
            host_element: None,
            date: String::new(),
            time: String::new(),
            cnt: 0,
            children: vec![],
            external_children_node: None,
        }
    }
}

impl DateTimeWidget
{
    pub fn new(date: &str, time: &str) -> Self {
        DateTimeWidget {
            date: date.to_string(),
            time: time.to_string(),
            ..Default::default()
        }
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

}

impl Component<Msg, ()> for DateTimeWidget{
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::DateChange(date) => {
                log::trace!("date is changed to: {}", date);
                self.date = date;
                Effects::with_local(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeChange(time) => {
                log::trace!("time is changed to: {}", time);
                self.time = time;
                Effects::with_local(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeOrDateModified(date_time) => {
                log::trace!("time or date is changed: {}", date_time);
                let mut parent_msg = vec![];
                if let Some(host_element) = self.host_element.as_ref() {
                    host_element
                        .set_attribute("date_time", &date_time)
                        .expect("change attribute");
                    host_element
                        .dispatch_event(&InputEvent::create_web_event_composed())
                        .expect("must dispatch event");
                } else {
                    log::debug!("There is no host_element");
                }
                Effects::with_external(parent_msg)
            }
            Msg::Mounted(mount_event) => {
                let mount_element: web_sys::Element = mount_event.target_node.unchecked_into();
                let root_node = mount_element.get_root_node();
                if let Some(shadow_root) = root_node.dyn_ref::<web_sys::ShadowRoot>() {
                    log::info!("There is a shadow root");
                    let host_element = shadow_root.host();
                    self.host_element = Some(host_element);
                } else {
                    log::warn!("There is no shadow root");
                }
                Effects::none()
            }
            Msg::ExternContMounted(target_node) => {
                log::info!("extenal container mounted...");
                for child in self.children.iter(){
                    target_node.append_child(child).expect("must append");
                }
                self.external_children_node = Some(target_node);
                Effects::none()
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                Effects::none()
            }
            Msg::IntervalChange(interval) => {
                log::trace!("There is an interval: {}", interval);
                Effects::none()
            }
        }
    }


    fn stylesheet() -> Vec<String> {
        vec![jss! {
            ".datetimebox":{
                border: "1px solid green",
            },
            "button": {
              background: "#1E88E5",
              color: "white",
              padding: "10px 10px",
              margin: "10px 10px",
              border: 0,
              font_size: "1.5rem",
              border_radius: "5px",
            }
        }]
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("datetimebox"), on_mount(Msg::Mounted)],
            [
                input(
                    [
                        r#type("date"),
                        class("datetimebox__date"),
                        on_change(|input| {
                            log::trace!("input: {:?}", input);
                            Msg::DateChange(input.value())
                        }),
                        value(&self.date),
                    ],
                    [],
                ),
                input(
                    [
                        r#type("time"),
                        class("datetimebox__time"),
                        on_change(|input| Msg::TimeChange(input.value())),
                        value(&self.time),
                    ],
                    [],
                ),
                input([r#type("text"), value(self.cnt)], []),
                button([on_click(move |_| Msg::BtnClick)], [text("Do something")]),
                div([class("external_children"), on_mount(|me|Msg::ExternContMounted(me.target_node))], [])
            ],
        )
    }
}

impl StatefulComponent for DateTimeWidget{

    fn build(
        attrs: impl IntoIterator<Item = DomAttr>,
        children: impl IntoIterator<Item = web_sys::Node>,
    ) -> Self
    where
        Self: Sized,
    {
        DateTimeWidget::default()
    }

    fn template(&self) -> web_sys::Node {
        template::build_template(&Component::view(self))
    }

    /// this is called when the attributes in the mount is changed
    fn attribute_changed(
        &mut self,
        attr_name: AttributeName,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized,
    {
        match &*attr_name {
            "time" => {
                if let Some(new_value) = new_value.get_string(){
                    Component::update(self, Msg::TimeChange(new_value));
                }
            }
            "date" => {
                if let Some(new_value) = new_value.get_string(){
                    Component::update(self, Msg::DateChange(new_value));
                }
            }
            _ => log::warn!("unknown attr_name: {attr_name:?}"),
        }
    }

    fn append_child(&mut self, child: &web_sys::Node) {
        log::info!("appending child to date_time: {:?}", child.inner_html());
        if let Some(external_children_node) = self.external_children_node.as_ref(){
            log::info!("ok appending..");
            external_children_node.append_child(child).expect("must append");
        }else{
            self.children.push(child.clone());
        }
    }

    fn remove_attribute(&mut self, attr_name: AttributeName) {}

    fn remove_child(&mut self, index: usize) {}

    fn connected_callback(&mut self) {}

    fn disconnected_callback(&mut self) {}

    fn adopted_callback(&mut self) {}
}

