use sauron::wasm_bindgen::JsCast;
use sauron::{
    custom_element, dom::Callback, html::attributes::*, html::events::*, html::*, jss,
    wasm_bindgen, web_sys, Attribute, Effects, JsValue, Node, WebComponent, *,
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    IntervalChange(f64),
    Mounted(MountEvent),
    BtnClick,
}

#[derive(Debug, Clone)]
pub struct DateTimeWidget<XMSG> {
    /// the host element the web editor is mounted to, when mounted as a custom web component
    host_element: Option<web_sys::Element>,
    date: String,
    time: String,
    cnt: i32,
    time_change_listener: Vec<Callback<String, XMSG>>,
}

impl<XMSG> Default for DateTimeWidget<XMSG> {
    fn default() -> Self {
        Self {
            host_element: None,
            date: String::new(),
            time: String::new(),
            cnt: 0,
            time_change_listener: vec![],
        }
    }
}

impl<XMSG> DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    pub fn new(date: &str, time: &str) -> Self {
        DateTimeWidget {
            host_element: None,
            date: date.to_string(),
            time: time.to_string(),
            cnt: 0,
            time_change_listener: vec![],
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

impl<XMSG> sauron::Component<Msg, XMSG> for DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    fn init(&mut self) -> Vec<Task<Msg>> {
        vec![]
    }

    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
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
    fn style(&self) -> Vec<String> {
        vec![]
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
                            Msg::DateChange(input.value)
                        }),
                        value(&self.date),
                    ],
                    [],
                ),
                input(
                    [
                        r#type("time"),
                        class("datetimebox__time"),
                        on_change(|input| Msg::TimeChange(input.value)),
                        value(&self.time),
                    ],
                    [],
                ),
                input([r#type("text"), value(self.cnt)], []),
                button([on_click(move |_| Msg::BtnClick)], [text("Do something")]),
            ],
        )
    }
}

#[custom_element("date-time")]
impl<XMSG> sauron::WebComponent<Msg> for DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    fn observed_attributes() -> Vec<&'static str> {
        vec!["date", "time", "interval"]
    }

    /// this is called when the attributes in the mount is changed
    fn attribute_changed(
        program: &Program<Self, Msg>,
        attr_name: &str,
        old_value: Option<String>,
        new_value: Option<String>,
    ) where
        Self: Sized + Application<Msg>,
    {
        log::info!("old_value: {:?}", old_value);
        log::info!("new_value: {new_value:?}");
        match &*attr_name {
            "time" => {
                if let Some(new_value) = new_value {
                    program.dispatch(Msg::TimeChange(new_value))
                }
            }
            "date" => {
                if let Some(new_value) = new_value {
                    program.dispatch(Msg::DateChange(new_value))
                }
            }
            "interval" => {
                if let Some(new_value) = new_value {
                    let new_value: f64 = str::parse(&new_value).expect("must parse to f64");
                    program.dispatch(Msg::IntervalChange(new_value))
                }
            }
            _ => log::warn!("unknown attr_name: {attr_name:?}"),
        }
    }

    fn connected_callback(&mut self) {}

    fn disconnected_callback(&mut self) {}

    fn adopted_callback(&mut self) {}
}

pub fn date<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("date", v)
}

pub fn time<MSG, V: Into<Value>>(v: V) -> Attribute<MSG> {
    attr("time", v)
}

pub fn date_time<MSG>(
    attrs: impl IntoIterator<Item = Attribute<MSG>>,
    children: impl IntoIterator<Item = Node<MSG>>,
) -> Node<MSG> {
    register();
    html_element(None, "date-time", attrs, children, true)
}
