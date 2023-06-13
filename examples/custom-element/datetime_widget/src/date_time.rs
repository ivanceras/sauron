use sauron::{
    dom::register_custom_element,
    dom::{Callback, MountAction, MountTarget},
    html::attributes::*,
    html::events::*,
    html::*,
    jss, wasm_bindgen, web_sys, Application, CustomElement, Dispatch, Effects, JsValue, Node,
    Program,
};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub enum Msg {
    DateChange(String),
    TimeChange(String),
    TimeOrDateModified(String),
    Mount(web_sys::Node),
    BtnClick,
}

#[derive(Debug, Clone)]
pub struct DateTimeWidget<XMSG> {
    date: String,
    time: String,
    cnt: i32,
    time_change_listener: Vec<Callback<String, XMSG>>,
}

impl<XMSG> Default for DateTimeWidget<XMSG> {
    fn default() -> Self {
        Self {
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
                Effects::with_external(parent_msg)
            }
            Msg::Mount(target_node) => {
                log::debug!("widget is mounted to {:?}", target_node);
                Effects::none()
            }
            Msg::BtnClick => {
                log::trace!("btn is clicked..");
                self.cnt += 1;
                Effects::none()
            }
        }
    }

    fn style(&self) -> String {
        jss! {
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
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [
                class("datetimebox"),
                on_mount(|me| Msg::Mount(me.target_node)),
            ],
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

impl<XMSG> sauron::CustomElement<Msg> for DateTimeWidget<XMSG>
where
    XMSG: 'static,
{
    fn observed_attributes() -> Vec<&'static str> {
        vec!["date", "time"]
    }

    /// this is called when the attributes in the mount is changed
    fn attribute_changed<DSP>(
        program: &DSP,
        attr_name: &str,
        old_value: JsValue,
        new_value: JsValue,
    ) where
        DSP: Dispatch<Msg> + Clone + 'static,
    {
        log::info!("old_value: {:?}", old_value);
        if let Some(new_value) = new_value.as_string() {
            match &*attr_name {
                "time" => program.dispatch(Msg::TimeChange(new_value)),
                "date" => program.dispatch(Msg::DateChange(new_value)),
                _ => log::warn!("unknown attr_name: {attr_name:?}"),
            }
        }
    }

    /// This is called when the attributes for the mount is to be set
    /// this is called every after update
    fn attributes_for_mount(&self) -> BTreeMap<String, String> {
        BTreeMap::from_iter([("value".to_string(), self.date_time())])
    }
}

#[wasm_bindgen]
pub struct DateTimeWidgetCustomElement {
    program: Program<DateTimeWidget<()>, Msg>,
}

#[wasm_bindgen]
impl DateTimeWidgetCustomElement {
    #[wasm_bindgen(constructor)]
    pub fn new(node: JsValue) -> Self {
        use sauron::wasm_bindgen::JsCast;
        let mount_node: &web_sys::Node = node.unchecked_ref();
        Self {
            program: Program::new(
                DateTimeWidget::<()>::default(),
                mount_node,
                MountAction::Append,
                MountTarget::ShadowRoot,
            ),
        }
    }

    #[wasm_bindgen(getter, static_method_of = Self, js_name = observedAttributes)]
    pub fn observed_attributes() -> JsValue {
        let attributes = DateTimeWidget::<Msg>::observed_attributes();
        JsValue::from_serde(&attributes).expect("must be serde")
    }

    #[wasm_bindgen(method, js_name = attributeChangedCallback)]
    pub fn attribute_changed_callback(
        &self,
        attr_name: &str,
        old_value: JsValue,
        new_value: JsValue,
    ) {
        log::info!(
            "attribute: {} is changed from: {:?} to: {:?}",
            attr_name,
            old_value,
            new_value
        );
        DateTimeWidget::<Msg>::attribute_changed(&self.program, attr_name, old_value, new_value);
    }

    #[wasm_bindgen(method, js_name = connectedCallback)]
    pub fn connected_callback(&mut self) {
        self.program.mount();
        let component_style =
            <DateTimeWidget<()> as Application<Msg>>::style(&self.program.app.borrow());
        self.program.inject_style_to_mount(&component_style);
        self.program.update_dom().expect("must update dom");
    }

    #[wasm_bindgen(method, js_name = disconnectedCallback)]
    pub fn disconnected_callback(&mut self) {}

    #[wasm_bindgen(method, js_name = adoptedCallback)]
    pub fn adopted_callback(&mut self) {}
}

pub fn register() {
    register_custom_element("date-time", "DateTimeWidgetCustomElement", "HTMLElement");
}
