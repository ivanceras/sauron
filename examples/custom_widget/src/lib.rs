mod date_time;

use date_time::DateTimeWidget;
use once_cell::sync::OnceCell;
use once_cell::unsync::Lazy;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Mutex;

static COMPONENT_ID: OnceCell<Mutex<usize>> = OnceCell::new();

fn create_unique_component_id() -> usize {
    let mut component_id = COMPONENT_ID
        .get_or_init(|| Mutex::new(0))
        .lock()
        .expect("unable to obtain lock");
    *component_id += 1;
    *component_id
}

pub struct Context<MSG, CMSG> {
    components: BTreeMap<usize, Box<dyn Component<CMSG, MSG>>>,
}

impl<MSG, CMSG> Context<MSG, CMSG> {
    fn new() -> Self {
        Self {
            components: BTreeMap::<usize, Box<dyn Component<CMSG, MSG>>>::new(),
        }
    }

    fn map_view<COMP, F>(&mut self, f: F, component: COMP) -> Node<MSG>
    where
        F: Fn(usize, CMSG) -> MSG + 'static,
        COMP: Component<CMSG, MSG> + 'static,
        MSG: 'static,
        CMSG: 'static,
    {
        let component_id = create_unique_component_id();
        let view = component.view().map_msg(move |cmsg| f(component_id, cmsg));
        self.components.insert(component_id, Box::new(component));
        view
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
    Mount(web_sys::Node),
    DateTimeMsg(usize, date_time::Msg),
    DateTimeChange(String),
}

pub struct App {
    count: i32,
    context: RefCell<Context<Msg, date_time::Msg>>,
}

impl App {
    pub fn new() -> Self {
        App {
            count: 0,
            context: RefCell::new(Context::new()),
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::none()
    }
    fn view(&self) -> Node<Msg> {
        let mut context = self.context.borrow_mut();
        div(
            vec![on_mount(|me| Msg::Mount(me.target_node))],
            vec![
                input(
                    vec![
                        r#type("button"),
                        value("+"),
                        key("inc"),
                        on_click(|_| Msg::Increment),
                    ],
                    vec![],
                ),
                div(vec![], vec![text(self.count)]),
                input(
                    vec![
                        r#type("button"),
                        value("-"),
                        key("dec"),
                        on_click(|_| Msg::Decrement),
                    ],
                    vec![],
                ),
                context.map_view(
                    |comp_id, comp_msg| Msg::DateTimeMsg(comp_id, comp_msg),
                    DateTimeWidget::new("2021-01-01", "11:11", false)
                        .on_date_time_change(Msg::DateTimeChange),
                ),
                context.map_view(
                    |comp_id, comp_msg| Msg::DateTimeMsg(comp_id, comp_msg),
                    DateTimeWidget::new("2022-02-02", "12:12", false)
                        .on_date_time_change(Msg::DateTimeChange),
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => {
                self.count += 1;
                Cmd::none()
            }
            Msg::Decrement => {
                self.count -= 1;
                Cmd::none()
            }
            Msg::Mount(target_node) => {
                log::trace!("app is mounted to {:?}", target_node);
                Cmd::none()
            }
            // this is only here for the purpose of mounting
            // the date time widget.
            // We want the date-time widget to have it's own lifecycle
            Msg::DateTimeMsg(comp_id, dmsg) => {
                let effects = self
                    .context
                    .borrow_mut()
                    .components
                    .get_mut(&comp_id)
                    .unwrap()
                    .update(dmsg);

                Cmd::from(
                    effects
                        .localize(move |dmsg| Msg::DateTimeMsg(comp_id, dmsg)),
                )
            }
            Msg::DateTimeChange(date_time) => {
                log::info!(
                    "From APP: DateTimeWidget has change the DateTime to: {}",
                    date_time
                );
                Cmd::none()
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
