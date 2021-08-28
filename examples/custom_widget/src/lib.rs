#![deny(warnings)]

use date_time::DateTimeWidget;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{Application, Cmd, Node};
use std::cell::RefCell;
use std::marker::PhantomData;

mod date_time;

pub struct Context<COMP, MSG, CMSG> {
    components: Vec<COMP>,
    _phantom_msg: PhantomData<MSG>,
    _phantom_cmsg: PhantomData<CMSG>,
}

impl<COMP, MSG, CMSG> Context<COMP, MSG, CMSG>
where
    COMP: Component<CMSG, MSG> + 'static,
    MSG: 'static,
    CMSG: 'static,
{
    fn new() -> Self {
        Self {
            components: vec![],
            _phantom_msg: PhantomData,
            _phantom_cmsg: PhantomData,
        }
    }

    /// simultaneously save the component into context for the duration until the next update loop
    fn map_view<F>(&mut self, mapper: F, component: COMP) -> Node<MSG>
    where
        F: Fn(usize, CMSG) -> MSG + 'static,
    {
        let component_id = self.components.len();
        log::trace!("component_id: {}", component_id);
        let view = component
            .view()
            .map_msg(move |cmsg| mapper(component_id, cmsg));
        self.components.push(component);
        view
    }

    fn update_component_with_id<F>(
        &mut self,
        comp_id: usize,
        cmsg: CMSG,
        mapper: F,
    ) -> Effects<MSG, ()>
    where
        F: Fn(CMSG) -> MSG + 'static,
    {
        self.components
            .get_mut(comp_id)
            .expect("component not found")
            .update(cmsg)
            .localize(mapper)
    }

    /// resets the components
    fn clear(&mut self) {
        self.components.clear();
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
    context: RefCell<Context<DateTimeWidget<Msg>, Msg, date_time::Msg>>,
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
        // we clear the components here as stateless components are recreated at every view call.
        context.clear();
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
                    Msg::DateTimeMsg,
                    DateTimeWidget::new("2021-01-01", "11:11", false)
                        .on_date_time_change(Msg::DateTimeChange),
                ),
                context.map_view(
                    Msg::DateTimeMsg,
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
                let mut context = self.context.borrow_mut();
                let effects = context.update_component_with_id(
                    comp_id,
                    dmsg,
                    move |dmsg| Msg::DateTimeMsg(comp_id, dmsg),
                );
                Cmd::from(effects)
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
