#![deny(warnings)]
use rand::prelude::*;
use sauron::dom::component;
use sauron::*;
use std::cmp::min;

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

#[derive(Clone, PartialEq)]
struct RowData {
    id: usize,
    label: String,
    selected: bool,
}

impl RowData {
    fn new(id: usize, rng: &mut SmallRng) -> Self {
        let adjective = *ADJECTIVES.choose(rng).unwrap();
        let colour = *COLOURS.choose(rng).unwrap();
        let noun = *NOUNS.choose(rng).unwrap();

        let label = [adjective, colour, noun].join(" ");

        Self {
            id,
            label,
            selected: false,
        }
    }
}

impl Component for RowData {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, _msg: Msg) -> Effects<Msg, ()> {
        Effects::none()
    }

    view! {
        <tr class={if self.selected { "danger" } else  { "" }} key=self.id >
            <td class="col-md-1">{ text(self.id) }</td>
            <td class="col-md-4"
            on_click={
                let id = self.id;
                move |_| Msg::Select(id)
            }>
                 <a class="lbl">{ text(&self.label) }</a>
            </td>
            <td class="col-md-1">
                 <a class="remove"
                 on_click={
                     let id = self.id;
                     move |_| Msg::Remove(id)
                 }>
                     <span class="glyphicon glyphicon-remove remove" aria-hidden="true"></span>
                 </a>
            </td>
            <td class="col-md-6"></td>
        </tr>
    }
}

struct App {
    rows: Vec<RowData>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: SmallRng,
}

impl App {
    fn new() -> Self {
        App {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
        }
    }
}

enum Msg {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Application for App {
    type MSG = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Run(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                let update_amount = min(amount, self.rows.len());
                for index in 0..update_amount {
                    self.rows[index] = RowData::new(next_id + index, rng);
                }
                self.rows.extend(
                    (update_amount..amount).map(|index| RowData::new(next_id + index, rng)),
                );
                self.next_id += amount;
            }
            Msg::Add(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                self.rows
                    .extend((0..amount).map(|index| RowData::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Update(step) => {
                for index in (0..self.rows.len()).step_by(step) {
                    self.rows[index].label += " !!!";
                }
            }
            Msg::Clear => {
                self.rows.clear();
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
            }
            Msg::Remove(id) => {
                if let Some(index) = self.rows.iter().position(|row| row.id == id) {
                    self.rows.remove(index);
                }
            }
            Msg::Select(id) => {
                self.selected_id = Some(id);
                for row in &mut self.rows {
                    row.selected = self.selected_id == Some(row.id)
                }
            }
        }
        Cmd::none()
    }

    view! {
        <div class="container">
             <div class="jumbotron">
                 <div class="row">
                     <div class="col-md-6">
                         <h1>Sauron</h1>
                     </div>
                     <div class="col-md-6">
                         <div class="row">
                            <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    id="run"
                                    class="btn btn-primary btn-block"
                                    on_click={|_| Msg::Run(10) }>
                                    Create 10 rows
                                </button>
                             </div>
                             <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    class="btn btn-primary btn-block"
                                    on_click={|_| {
                                        log::info!("Creating 10_000 rows...");
                                        Msg::Run(10_000)
                                    }}
                                    id="runlots">
                                    Create 10,000 rows
                                </button>
                             </div>
                             <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    class="btn btn-primary btn-block"
                                    on_click={ |_| Msg::Add(1_000)} id="add">
                                    Append 1,000 rows
                                </button>
                             </div>
                             <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    class="btn btn-primary btn-block"
                                    on_click={|_|Msg::Update(10)}
                                    id="update">
                                    Update every 10th row
                                </button>
                             </div>
                             <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    class="btn btn-primary btn-block"
                                    on_click={|_|Msg::Clear} id="clear">
                                    Clear
                                </button>
                             </div>
                             <div class="col-sm-6 smallpad">
                                <button
                                    type="button"
                                    class="btn btn-primary btn-block"
                                    on_click={ |_|Msg::Swap }
                                    id="swaprows">
                                    Swap Rows
                                </button>
                             </div>
                         </div>
                     </div>
                 </div>
             </div>
            <table class="table table-hover table-striped test-data">
                <tbody id="tbody">
                    {for row in self.rows.iter() {
                        component(row, [], [])
                    }}
                </tbody>
            </table>
            <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true"></span>
        </div>
    }
}


#[wasm_bindgen(start)]
pub fn start() {
    console_log::init().unwrap();
    console_error_panic_hook::set_once();

    let mount_el = sauron::document().query_selector("#main").unwrap().unwrap();
    Program::append_to_mount(App::new(), &mount_el);
}
