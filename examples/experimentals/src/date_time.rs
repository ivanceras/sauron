use sauron::dom::DomAttr;
use sauron::dom::DomAttrValue;
use sauron::dom::DomNode;
use sauron::dom::MountProcedure;
use sauron::dom::StatefulComponent;
use sauron::vdom::AttributeName;
use sauron::wasm_bindgen::JsCast;
use sauron::{
    html::attributes::*, html::events::*, html::*, jss, vdom::Callback,
    wasm_bindgen, web_sys, Attribute, Effects, JsValue, Node, *,
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    IntervalChange(f64),
    Mounted(MountEvent),
    ExternContMounted(DomNode),
    BtnClick,
    NoOp,
}

#[derive(Debug, Clone)]
pub struct DateTimeWidget<XMSG> {
    /// the host element the web editor is mounted to, when mounted as a custom web component
    host_element: Option<DomNode>,
    date: String,
    time: String,
    cnt: i32,
    time_change_listener: Vec<Callback<String, XMSG>>,
    children: Vec<DomNode>,
    external_children_node: Option<DomNode>,
}

impl<XMSG> Default for DateTimeWidget<XMSG> {
    fn default() -> Self {
        Self {
            host_element: None,
            date: String::new(),
            time: String::new(),
            cnt: 0,
            time_change_listener: vec![],
            children: vec![],
            external_children_node: None,
        }
    }
}

impl<XMSG> DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    pub fn new(date: &str, time: &str) -> Self {
        DateTimeWidget {
            date: date.to_string(),
            time: time.to_string(),
            cnt: 0,
            ..Default::default()
        }
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

    pub fn on_date_time_change<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> XMSG + 'static,
    {
        self.time_change_listener.push(Callback::from(f));
        self
    }
}

impl<XMSG> sauron::Component for DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    type MSG = Msg;
    type XMSG = XMSG;

    fn update(&mut self, msg: Msg) -> Effects<Msg, Self::XMSG> {
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
                for listener in self.time_change_listener.iter() {
                    let pmsg = listener.emit(self.date_time());
                    parent_msg.push(pmsg);
                }
                if let Some(host_element) = self.host_element.as_ref() {
                    /*
                    host_element
                        .set_attribute("date_time", &date_time)
                        .expect("change attribute");
                    host_element
                        .dispatch_event(&InputEvent::create_web_event_composed())
                        .expect("must dispatch event");
                    */
                } else {
                    log::debug!("There is no host_element");
                }
                Effects::with_external(parent_msg)
            }
            Msg::Mounted(mount_event) => {
                log::info!("==> Ok the DateTime widget is now mounted for real..");
                let mount_element = mount_event.target_node;
                self.host_element = Some(mount_element);
                Effects::none()
            }
            Msg::ExternContMounted(target_node) => {
                log::warn!("-->>> Container for children is now mounted..!");
                target_node.append_children(self.children.drain(..).collect());
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
            Msg::NoOp => Effects::none(),
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

    fn observed_attributes() -> Vec<AttributeName> {
        vec!["date", "time", "interval"]
    }

    view! {
        <div class="datetimebox" on_mount=Msg::Mounted>
            <input type="date" class="datetimebox__date"
                        on_change=|input| {
                            log::trace!("input: {:?}", input);
                            Msg::DateChange(input.value())
                        }
                        value=&self.date/>
            <input type="time"
                    class="datetimebox__time"
                    on_change=|input|Msg::TimeChange(input.value())
                    value=&self.time/>
            <input type="text" value=self.cnt/>
            <button on_click=move |_| Msg::BtnClick on_mount=|me|{log::info!("button is mounted...");Msg::NoOp}>Do something</button>
            <div class="external_children" on_mount=|me|Msg::ExternContMounted(me.target_node)>Here is something!</div>
        </div>
    }
}

impl StatefulComponent for DateTimeWidget<()> {
    /// this is called when the attributes in the mount is changed
    fn attribute_changed(
        &mut self,
        attr_name: &str,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized,
    {
        match &*attr_name {
            "time" => {
                if let Some(new_value) = new_value.get_string() {
                    Component::update(self, Msg::TimeChange(new_value));
                }
            }
            "date" => {
                if let Some(new_value) = new_value.get_string() {
                    Component::update(self, Msg::DateChange(new_value));
                }
            }
            "interval" => {
                if let Some(new_value) = new_value.get_string() {
                    let new_value: f64 = str::parse(&new_value).expect("must parse to f64");
                    Component::update(self, Msg::IntervalChange(new_value));
                }
            }
            _ => log::warn!("unknown attr_name: {attr_name:?}"),
        }
    }

    fn append_children(&mut self, children: Vec<DomNode>) {
        if let Some(external_children_node) = self.external_children_node.as_ref() {
            log::info!("DateTime: ok appending..");
            external_children_node.append_children(children);
        } else {
            log::debug!(
                "DateTime: Just pushing to children since the external holder is not yet mounted"
            );
            self.children.extend(children);
        }
    }
    fn child_container(&self) -> Option<DomNode> {
        todo!()
    }
}

pub fn date<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("date", v)
}

pub fn time<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("time", v)
}

pub fn date_time<MSG:'static>(
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG> {
    stateful_component(DateTimeWidget::default(), attrs, children)
}
