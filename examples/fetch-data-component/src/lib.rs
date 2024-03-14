#![deny(warnings)]
use fetcher::Fetcher;
use sauron::*;

#[macro_use]
extern crate log;

mod fetcher;

#[derive(Debug)]
pub enum Msg {
    FetcherMsg(fetcher::Msg),
}

pub struct App {
    fetcher: Fetcher,
}

impl App {
    pub fn new() -> Self {
        App {
            fetcher: Fetcher::new(),
        }
    }
}

impl Application for App {
    
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self> {
        console_log::init_with_level(log::Level::Trace).unwrap();
        Cmd::from(self.fetcher.init().map_msg(Msg::FetcherMsg))
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>This is the app</h1>
                {self.fetcher.view().map_msg(Msg::FetcherMsg)}
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::FetcherMsg(fmsg) => Cmd::from(self.fetcher.update(fmsg).map_msg(Msg::FetcherMsg)),
        }
    }

    fn stylesheet() -> Vec<String> {
        Fetcher::stylesheet()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}

#[cfg(test)]
mod tests {
    use crate::fetcher::Data;

    #[test]
    fn test_json() {
        let json = r#"
{"page":1,"per_page":3,"total":12,"total_pages":4,"data":[{"id":1,"email":"george.bluth@reqres.in","first_name":"George","last_name":"Bluth","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/calebogden/128.jpg"},{"id":2,"email":"janet.weaver@reqres.in","first_name":"Janet","last_name":"Weaver","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/josephstein/128.jpg"},{"id":3,"email":"emma.wong@reqres.in","first_name":"Emma","last_name":"Wong","avatar":"https://s3.amazonaws.com/uifaces/faces/twitter/olegpogodaev/128.jpg"}]}
        "#;
        println!("json: {}", json);
        let data: Result<Data, _> = serde_json::from_str(json);
        println!("data: {:#?}", data);
        assert!(data.is_ok());
    }
}
