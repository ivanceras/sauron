use sauron::dom::DomNode;
use sauron::vdom::Callback;
use sauron::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    Mounted(MountEvent),
}

#[derive(Debug, Clone)]
pub struct DateBox<XMSG> {
    /// the host element the web editor is mounted to, when mounted as a custom web component
    host_element: Option<DomNode>,
    date: String,
    time: String,
    time_change_listener: Vec<Callback<String, XMSG>>,
}

impl<XMSG> Default for DateBox<XMSG> {
    fn default() -> Self {
        Self {
            host_element: None,
            date: String::new(),
            time: String::new(),
            time_change_listener: vec![],
        }
    }
}

impl<XMSG> DateBox<XMSG>
where
    XMSG: 'static,
{
    #[allow(unused)]
    pub fn new(date: &str, time: &str) -> Self {
        DateBox {
            date: date.to_string(),
            time: time.to_string(),
            ..Default::default()
        }
    }

    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }

    #[allow(unused)]
    pub fn on_date_time_change<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> XMSG + 'static,
    {
        self.time_change_listener.push(Callback::from(f));
        self
    }
}

impl<XMSG> sauron::Component for DateBox<XMSG>
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
                    host_element
                        .as_element()
                        .set_attribute("date_time", &date_time)
                        .expect("change attribute");
                    host_element
                        .as_element()
                        .dispatch_event(&InputEvent::create_web_event_composed())
                        .expect("must dispatch event");
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
        }
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
        </div>
    }
}

impl StatefulComponent for DateBox<()> {
    /// this is called when the attributes in the mount is changed
    fn attribute_changed(&mut self, attr_name: &str, new_value: Vec<DomAttrValue>) {
        match attr_name {
            "time" => {
                if let Some(new_value) = new_value[0].as_string() {
                    Component::update(self, Msg::TimeChange(new_value));
                }
            }
            "date" => {
                if let Some(new_value) = new_value[0].as_string() {
                    Component::update(self, Msg::DateChange(new_value));
                }
            }
            _ => log::warn!("unknown attr_name: {attr_name:?}"),
        }
    }

    fn child_container(&self) -> Option<DomNode> {
        None
    }
}

pub fn date<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("date", v)
}

pub fn time<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("time", v)
}

pub fn datebox<MSG: 'static>(
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG> {
    stateful_component(DateBox::default(), attrs, children)
}
