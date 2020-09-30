use sauron::{
    dom::events::KeyboardEvent,
    html::{
        attributes::*,
        *,
    },
    prelude::*,
    Cmd,
    Component,
    Node,
};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Model {
    entries: Vec<Entry>,
    visibility: Visibility,
    value: String,
    uid: usize,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
    id: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Visibility {
    All,
    Active,
    Completed,
}

pub enum Msg {
    Add,
    EditingEntry(usize, bool),
    Update(String),
    UpdateEntry(usize, String),
    Delete(usize),
    ChangeVisibility(Visibility),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    NoOp,
}

impl Component<Msg> for Model {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Add => {
                self.entries.push(Entry::new(&self.value, self.uid));
                self.uid += 1;
                self.value = "".to_string();
            }
            Msg::EditingEntry(id, is_editing) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = is_editing;
                    }
                });
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::UpdateEntry(id, new_description) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.description = new_description.clone();
                    }
                });
            }
            Msg::Delete(id) => {
                self.entries.retain(|entry| entry.id != id);
            }
            Msg::ChangeVisibility(visibility) => {
                self.visibility = visibility;
            }
            Msg::ToggleEdit(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = !entry.editing;
                    }
                });
            }
            Msg::ToggleAll => {
                let is_all_completed = !self.is_all_completed();
                self.entries
                    .iter_mut()
                    .for_each(|entry| entry.completed = is_all_completed);
            }
            Msg::Toggle(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.completed = !entry.completed;
                    }
                });
            }
            Msg::ClearCompleted => {
                self.entries.retain(|entry| !entry.completed);
            }
            Msg::NoOp => {}
        }
        #[cfg(feature = "with-storage")]
        self.save_to_storage();
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("todomvc-wrapper")],
            vec![
                section(
                    vec![class("todoapp")],
                    vec![
                        self.view_input(),
                        self.view_entries(),
                        self.view_controls(),
                    ],
                ),
                self.info_footer(),
            ],
        )
    }
}

impl Entry {
    fn new(description: &str, id: usize) -> Self {
        Entry {
            description: description.to_string(),
            completed: false,
            editing: false,
            id,
        }
    }
}

impl Model {
    pub(crate) fn new() -> Self {
        Model {
            entries: vec![],
            visibility: Visibility::All,
            value: "".into(),
            uid: 0,
        }
    }

    fn is_all_completed(&self) -> bool {
        self.entries.iter().all(|entry| entry.completed)
    }

    fn view_entries(&self) -> Node<Msg> {
        section(
            vec![class("main")],
            vec![
                input(
                    vec![
                        class("toggle-all"),
                        r#type("checkbox"),
                        checked(self.is_all_completed()),
                        on_click(|_| Msg::ToggleAll),
                    ],
                    vec![],
                ),
                ul(vec![class("todo-list")], {
                    self.entries
                        .iter()
                        .filter(|entry| {
                            match self.visibility {
                                Visibility::All => true,
                                Visibility::Active => !entry.completed,
                                Visibility::Completed => entry.completed,
                            }
                        })
                        .map(|entry| self.view_entry(entry))
                        .collect::<Vec<Node<Msg>>>()
                }),
            ],
        )
    }

    fn view_filter(&self, visibility: Visibility) -> Node<Msg> {
        let visibility_str = visibility.to_string();
        li(
            vec![],
            vec![a(
                vec![
                    class(if self.visibility == visibility {
                        "selected"
                    } else {
                        "not-selected"
                    }),
                    href(visibility.to_uri()),
                    on_click(move |_| {
                        Msg::ChangeVisibility(visibility.clone())
                    }),
                ],
                vec![text(visibility_str)],
            )],
        )
    }

    fn view_input(&self) -> Node<Msg> {
        header(
            vec![class("header")],
            vec![
                h1(vec![], vec![text("todos")]),
                input(
                    vec![
                        class("new-todo"),
                        id("new-todo"),
                        placeholder("What needs to be done?"),
                        autofocus(true),
                        value(self.value.to_string()),
                        on_input(|v: InputEvent| {
                            Msg::Update(v.value.to_string())
                        }),
                        on_keypress(|event: KeyboardEvent| {
                            if event.key() == "Enter" {
                                Msg::Add
                            } else {
                                Msg::NoOp
                            }
                        }),
                    ],
                    vec![],
                ),
            ],
        )
    }

    fn view_entry(&self, entry: &Entry) -> Node<Msg> {
        let mut class_name = "todo".to_string();
        if entry.editing {
            class_name.push_str(" editing");
        }
        if entry.completed {
            class_name.push_str(" completed");
        }
        let entry_id = entry.id;
        li(
            vec![class(class_name), key(format!("todo-{}", entry.id))],
            vec![
                div(
                    vec![class("view")],
                    vec![
                        input(
                            vec![
                                class("toggle"),
                                r#type("checkbox"),
                                checked(entry.completed),
                                on_click(move |_| Msg::Toggle(entry_id)),
                            ],
                            vec![],
                        ),
                        label(
                            vec![on_doubleclick(move |_| {
                                Msg::ToggleEdit(entry_id)
                            })],
                            vec![text(format!("{}", entry.description))],
                        ),
                        button(
                            vec![
                                class("destroy"),
                                on_click(move |_| Msg::Delete(entry_id)),
                            ],
                            vec![],
                        ),
                    ],
                ),
                input(
                    vec![
                        class("edit"),
                        r#type("text"),
                        hidden(!entry.editing),
                        value(&entry.description),
                        on_input(move |input: InputEvent| {
                            Msg::UpdateEntry(entry_id, input.value.to_string())
                        }),
                        on_blur(move |_| Msg::EditingEntry(entry_id, false)),
                        on_keypress(move |event: KeyboardEvent| {
                            if event.key_code() == 13 {
                                Msg::EditingEntry(entry_id, false)
                            } else {
                                Msg::NoOp
                            }
                        }),
                    ],
                    vec![],
                ),
            ],
        )
    }

    fn view_controls(&self) -> Node<Msg> {
        let entries_completed =
            self.entries.iter().filter(|entry| entry.completed).count();

        let entries_left = self.entries.len() - entries_completed;
        let item = if entries_left == 1 { " item" } else { " items" };

        footer(
            vec![class("footer")],
            vec![
                span(
                    vec![class("todo-count")],
                    vec![
                        strong(vec![], vec![text(entries_left)]),
                        text(format!(" {} left", item)),
                    ],
                ),
                ul(
                    vec![class("filters")],
                    vec![
                        self.view_filter(Visibility::All),
                        self.view_filter(Visibility::Active),
                        self.view_filter(Visibility::Completed),
                    ],
                ),
                button(
                    vec![
                        class("clear-completed"),
                        hidden(entries_completed == 0),
                        on_click(|_| Msg::ClearCompleted),
                    ],
                    vec![text(format!(
                        "Clear completed ({})",
                        entries_completed
                    ))],
                ),
            ],
        )
    }

    fn info_footer(&self) -> Node<Msg> {
        footer(
            vec![class("info")],
            vec![
                p(vec![], vec![text("Double-click to edit a todo")]),
                p(
                    vec![],
                    vec![
                        text("Written by "),
                        a(
                            vec![
                                href("https://github.com/ivanceras/"),
                                target("_blank"),
                            ],
                            vec![text("Jovansonlee Cesar")],
                        ),
                    ],
                ),
                p(
                    vec![],
                    vec![
                        text("Part of "),
                        a(
                            vec![href("http://todomvc.com/"), target("_blank")],
                            vec![text("TodoMVC")],
                        ),
                    ],
                ),
            ],
        )
    }

    #[allow(unused)]
    fn save_to_storage(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage();
        if let Ok(Some(local_storage)) = local_storage {
            let json_data =
                serde_json::to_string(&self).expect("must serialize data");
            if let Err(err) =
                local_storage.set_item("todomvc::data", &json_data)
            {
                log::error!("Could not write to local storage, {:?}", err);
            }
        }
    }

    #[allow(unused)]
    pub fn get_from_storage() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let local_storage = window.local_storage();

        if let Ok(Some(local_storage)) = local_storage {
            if let Ok(Some(s)) = local_storage.get_item("todomvc::data") {
                serde_json::from_str(&s).ok().unwrap_or(Self::new())
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Visibility::All => "All".to_string(),
            Visibility::Active => "Active".to_string(),
            Visibility::Completed => "Completed".to_string(),
        }
    }
}

impl Visibility {
    fn to_uri(&self) -> String {
        match self {
            Visibility::All => "#/".to_string(),
            Visibility::Active => "#/active".to_string(),
            Visibility::Completed => "#/completed".to_string(),
        }
    }
}
