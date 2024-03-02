use sauron::{dom::Http, js_sys::TypeError, jss, *};
use serde::Deserialize;

const DATA_URL: &str = "https://reqres.in/api/users";
const PER_PAGE: i32 = 4;

#[derive(Debug)]
pub enum Msg {
    NextPage,
    PrevPage,
    ReceivedData(Data),
    JsonError(serde_json::Error),
    RequestError(TypeError),
}

pub struct Fetcher {
    page: i32,
    data: Data,
    error: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Data {
    page: i32,
    per_page: i32,
    total: i32,
    total_pages: i32,
    data: Vec<User>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
    avatar: String,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            page: 1,
            data: Data::default(),
            error: None,
        }
    }

    fn fetch_page(&self) -> Effects<Msg, ()> {
        let url = format!("{}?page={}&per_page={}", DATA_URL, self.page, PER_PAGE);

        Effects::with_local_async([async move {
            match Http::fetch_text(&url).await {
                Ok(v) => match serde_json::from_str(&v) {
                    Ok(data1) => Msg::ReceivedData(data1),
                    Err(err) => Msg::JsonError(err),
                },
                Err(e) => Msg::RequestError(e),
            }
        }])
    }
}

impl Component<Msg, ()> for Fetcher {
    fn init(&mut self) -> Effects<Msg, ()> {
        self.fetch_page()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <div>
                 <div class="some-class" id="some-id" {attr("data-id", 1)}>
                         <input class="prev_page" type="button"
                                 disabled={self.page <= 1}
                                 value="<< Prev Page"
                                 on_click=|_| {
                                     trace!("Button is clicked");
                                     Msg::PrevPage
                                 }
                         />
                         {text(format!("Page: {}", self.page))}
                         <input class="next_page" type="button"
                                 disabled={self.page >= self.data.total_pages}
                                 value="Next Page >>"
                                 on_click=|_|{
                                     trace!("Button is clicked");
                                     Msg::NextPage
                                 }
                         />
                 </div>
                 <div>
                     {
                         for user in self.data.data.iter(){
                             node!{
                                 <ul>
                                     <li>{text(&user.id)}</li>
                                     <li>{text(&user.email)}</li>
                                     <li>{text(&user.first_name)}</li>
                                     <li><img src=&user.avatar/></li>
                                 </ul>
                             }
                         }
                     }
                 </div>
                 <footer class="error">
                     {if let Some(error) = &self.error {
                         text(error)
                     } else {
                         text!("")
                     }}
                 </footer>
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        trace!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::NextPage => {
                if self.page < self.data.total_pages {
                    self.page += 1;
                    self.fetch_page()
                } else {
                    Effects::none()
                }
            }
            Msg::PrevPage => {
                if self.page > 1 {
                    self.page -= 1;
                }
                self.fetch_page()
            }
            Msg::ReceivedData(data1) => {
                self.data = data1;
                Effects::none()
            }
            Msg::JsonError(err) => {
                trace!("Error fetching users! {:#?}", err);
                self.error = Some(format!("There was an error fetching the page: {:?}", err));
                Effects::none()
            }
            Msg::RequestError(type_error) => {
                trace!("Error requesting the page: {:?}", type_error);
                self.error = Some(format!(
                    "There was an error fetching the page: {:?}",
                    type_error
                ));
                Effects::none()
            }
        }
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}
