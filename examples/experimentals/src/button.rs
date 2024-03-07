
use sauron::prelude::*;
use sauron::dom::Component;


pub enum Msg{
    Click,
}

#[derive(Default)]
pub struct Button{
    cnt: i32,
}

impl Component<Msg, ()> for Button {

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg{
            Msg::Click => self.cnt +=1,
        }
        Effects::none()
    }

    fn view(&self) -> Node<Msg>{
        node!{
            <button on_click=|_|Msg::Click >Hello!{text!("I'm just a button, clicked {} time(s)", self.cnt)}</button>
        }
    }
}
