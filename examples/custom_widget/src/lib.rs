#![deny(warnings)]

use date_time::DateTimeWidget;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{Application, Cmd, Node};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

mod date_time;

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
    Mount(web_sys::Node),
    DateTimeMsg(Rc<RefCell<DateTimeWidget<Msg>>>, date_time::Msg),
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
        div(
            [on_mount(|me| Msg::Mount(me.target_node))],
            [
                input(
                    [
                        r#type("button"),
                        value("+"),
                        key("inc"),
                        on_click(|_| Msg::Increment),
                    ],
                    [],
                ),
                div([], [text(self.count)]),
                input(
                    [
                        r#type("button"),
                        value("-"),
                        key("dec"),
                        on_click(|_| Msg::Decrement),
                    ],
                    [],
                ),
                context.map_view(
                    DateTimeWidget::new("2021-01-01", "11:11")
                        .on_date_time_change(Msg::DateTimeChange),
                    Msg::DateTimeMsg,
                ),
                context.map_view(
                    DateTimeWidget::new("2022-02-02", "12:12")
                        .on_date_time_change(Msg::DateTimeChange),
                    Msg::DateTimeMsg,
                ),
                context.map_view(
                    DateTimeWidget::new("3033-03-03", "13:13")
                        .on_date_time_change(Msg::DateTimeChange),
                    Msg::DateTimeMsg,
                ),
                context.map_view(
                    DateTimeWidget::new("4044-04-04", "14:14")
                        .on_date_time_change(Msg::DateTimeChange),
                    Msg::DateTimeMsg,
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
            Msg::DateTimeMsg(component, dmsg) => {
                let effects = self.context.borrow_mut().update_component(
                    component,
                    dmsg,
                    Msg::DateTimeMsg,
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

pub struct Context<COMP, MSG, CMSG> {
    components: Vec<Rc<RefCell<COMP>>>,
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
    fn map_view<F>(&mut self, component: COMP, mapper: F) -> Node<MSG>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
    {
        let component = Rc::new(RefCell::new(component));
        let component_clone = component.clone();
        let view = component
            .borrow()
            .view()
            .map_msg(move |cmsg| mapper(component_clone.clone(), cmsg));
        self.components.push(component);
        view
    }

    fn update_component<F>(
        &mut self,
        component: Rc<RefCell<COMP>>,
        dmsg: CMSG,
        mapper: F,
    ) -> Effects<MSG, ()>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
    {
        let component_clone = component.clone();
        component
            .borrow_mut()
            .update(dmsg)
            .localize(move |dmsg| mapper(component_clone.clone(), dmsg))
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
