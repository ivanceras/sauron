use sauron::dom::DomAttr;
use sauron::dom::DomNode;
use sauron::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    Mounted(MountEvent),
    NoOp,
}

#[derive(Debug)]
pub struct DateBox {
    /// the host element the web editor is mounted to, when mounted as a custom web component
    host_element: Option<DomNode>,
    dom_attrs: Vec<DomAttr>,
    date: String,
    time: String,
}

impl Default for DateBox {
    fn default() -> Self {
        Self {
            host_element: None,
            dom_attrs: vec![],
            date: String::new(),
            time: String::new(),
        }
    }
}

impl DateBox {
    fn date_time(&self) -> String {
        format!("{} {}", self.date, self.time)
    }
}

impl sauron::Component for DateBox {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::DateChange(date) => {
                self.date = date;
                Effects::with_local(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeChange(time) => {
                self.time = time;
                Effects::with_local(vec![Msg::TimeOrDateModified(self.date_time())])
            }
            Msg::TimeOrDateModified(date_time) => {
                if let Some(host_element) = self.host_element.as_ref() {
                    log::info!("setting date_time..");
                    host_element
                        .as_element()
                        .set_attribute("value", &date_time)
                        .expect("change attribute");
                    log::info!("dispatching input event..");
                    host_element
                        .as_element()
                        .dispatch_event(&InputEvent::create_web_event())
                        .expect("must dispatch event");
                } else {
                    log::debug!("There is no host_element");
                }
                Effects::none()
            }
            Msg::Mounted(mount_event) => {
                log::info!("==> Ok the DateTime widget is now mounted for real..");
                let mount_element = mount_event.target_node;
                mount_element
                    .set_dom_attrs(self.dom_attrs.drain(..))
                    .unwrap();
                self.host_element = Some(mount_element);
                Effects::none()
            }
            Msg::NoOp => Effects::none(),
        }
    }

    view! {
        <div class="datetimebox" on_mount=Msg::Mounted>
            <input type="date" class="datetimebox__date"
                        on_change=|input|{
                            input.stop_propagation();
                            Msg::NoOp
                        }
                        on_input=|input| {
                            input.stop_propagation();
                            log::trace!("triggered from date..");
                            Msg::DateChange(input.value())
                        }
                        value=&self.date/>
            <input type="time"
                    class="datetimebox__time"
                    on_change=|input|{
                        input.stop_propagation();
                        Msg::NoOp
                    }
                    on_input=|input|{
                        log::trace!("triggered from time..");
                        input.stop_propagation();
                        Msg::TimeChange(input.value())
                    }
                    value=&self.time/>
        </div>
    }
}

impl StatefulComponent for DateBox {
    /// this is called when the attributes in the mount is changed
    fn attribute_changed(&mut self, attr: DomAttr) {
        match attr.name {
            "time" => {
                if let Some(new_value) = attr.value[0].as_string() {
                    Component::update(self, Msg::TimeChange(new_value));
                }
            }
            "date" => {
                if let Some(new_value) = attr.value[0].as_string() {
                    Component::update(self, Msg::DateChange(new_value));
                }
            }
            _ => {
                log::warn!("unknown attr_name: {attr:?}");
                self.dom_attrs.push(attr);
            }
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
