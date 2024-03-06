
use sauron::prelude::*;


pub enum Msg{
    Click,
}

#[derive(Default)]
pub struct Button{
    cnt: i32,
}

impl Application<Msg> for Button {

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg{
            Msg::Click => self.cnt +=1,
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg>{
        node!{
            <button on_click=|_|Msg::Click >Hello!{text!("I'm just a button, clicked {} time(s)", self.cnt)}</button>
        }
    }
}
