use sauron::html::{attributes::*, *};
use sauron::*;

pub struct Model {
    entries: Vec<Entry>,
    visibility: Visibility,
    field: String,
    uid: usize,
}

struct Entry {
    description: String,
    completed: bool,
    editing: bool,
    id: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Visibility {
    All,
    Active,
    Completed,
}

pub enum Msg {
    NoOp,
    UpdateField(String),
    EditingEntry(usize),
    UpdateEntry(usize, String),
    Add,
    Delete(usize),
    DeleteComplete,
    Check(usize, bool),
    CheckAll(bool),
    ChangeVisibility(Visibility),
}

fn on_enter<F>(f: F) -> Attribute<Msg>
where
    F: Fn() -> Msg + 'static,
{
    on_keydown(move |ke| if ke.key_code() == 13 { f() } else { Msg::NoOp })
}

impl Model {
    pub fn new() -> Self {
        Model {
            entries: vec![],
            visibility: Visibility::All,
            field: "".into(),
            uid: 0,
        }
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
                        value(&self.field),
                        name("newTodo"),
                        on_input(|input| {
                            Msg::UpdateField(input.value.to_string())
                        }),
                        on_enter(|| Msg::Add),
                    ],
                    vec![],
                ),
            ],
        )
    }

    fn view_entries(&self) -> Node<Msg> {
        let all_completed = self.entries.iter().all(|entry| entry.completed);
        section(
            vec![
                class("main"),
                styles([(
                    "visibility",
                    if self.entries.is_empty() {
                        "hidden"
                    } else {
                        "visible"
                    },
                )]),
            ],
            vec![
                input(
                    vec![
                        class("toggle-all"),
                        type_("checkbox"),
                        name("toggle"),
                        checked(all_completed),
                        on_click(move |_| Msg::CheckAll(!all_completed)),
                    ],
                    vec![],
                ),
                label(
                    vec![for_("toggle-all")],
                    vec![text("Mark all as complete")],
                ),
                ul(
                    vec![class("todo-list")],
                    self.entries
                        .iter()
                        .filter(|entry| match self.visibility {
                            Visibility::Completed => entry.completed,
                            Visibility::Active => !entry.completed,
                            Visibility::All => true,
                        })
                        .map(|entry| self.view_entry(entry))
                        .collect(),
                ),
            ],
        )
    }

    fn view_entry(&self, entry: &Entry) -> Node<Msg> {
        let entry_id = entry.id;
        let entry_completed = entry.completed;
        let entry_description = entry.description.clone();
        let entry_editing = entry.editing;
        li(
            vec![classes_flag([
                ("completed", entry_completed),
                ("editing", entry_editing),
            ])],
            vec![
                div(
                    vec![class("view")],
                    vec![
                        input(
                            vec![
                                class("toggle"),
                                type_("checkbox"),
                                checked(entry_completed),
                                on_click(move |_| {
                                    Msg::Check(entry_id, !entry_completed)
                                }),
                            ],
                            vec![],
                        ),
                        label(
                            vec![on_doubleclick(move |_| {
                                Msg::EditingEntry(entry_id)
                            })],
                            vec![text(&entry_description)],
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
                        value(&entry_description),
                        name("title"),
                        id(format!("todo-{}", entry_id)),
                        key(format!("todo-{}", entry_id)),
                        on_input(move |input| {
                            Msg::UpdateEntry(entry_id, input.value.to_string())
                        }),
                        on_enter(move || Msg::EditingEntry(entry_id)),
                    ],
                    vec![],
                ),
            ],
        )
    }

    fn view_controls(&self) -> Node<Msg> {
        let entries_completed = self.entries.iter().fold(0, |acc, entry| {
            if entry.completed {
                acc + 1
            } else {
                acc
            }
        });
        let entries_left = self.entries.len() - entries_completed;

        footer(
            vec![class("footer")],
            vec![
                self.view_controls_count(entries_left),
                self.view_controls_filters(),
                self.view_controls_clear(entries_completed),
            ],
        )
    }

    fn view_controls_count(&self, entries_left: usize) -> Node<Msg> {
        let item = if entries_left == 1 { " item" } else { " items" };
        span(
            vec![class("todo-count")],
            vec![
                strong(vec![], vec![text(entries_left)]),
                text(format!("{} left", item)),
            ],
        )
    }

    fn view_controls_filters(&self) -> Node<Msg> {
        ul(
            vec![class("filters")],
            vec![
                self.visibility_swap(Visibility::All),
                text(" "),
                self.visibility_swap(Visibility::Active),
                text(" "),
                self.visibility_swap(Visibility::Completed),
            ],
        )
    }

    fn visibility_swap(&self, visibility: Visibility) -> Node<Msg> {
        let visibility_uri = visibility.to_uri();
        let visibility_str = visibility.to_string();
        let is_selected = visibility == self.visibility;
        li(
            vec![on_click(move |_| Msg::ChangeVisibility(visibility.clone()))],
            vec![a(
                vec![
                    href(visibility_uri),
                    classes_flag([("selected", is_selected)]),
                ],
                vec![text(visibility_str)],
            )],
        )
    }

    fn view_controls_clear(&self, entries_completed: usize) -> Node<Msg> {
        button(
            vec![
                class("clear-completed"),
                hidden(entries_completed == 0),
                on_click(|_| Msg::DeleteComplete),
            ],
            vec![text(format!("Clear completed ({})", entries_completed))],
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
                            vec![href("https://github.com/ivanceras")],
                            vec![text("Jovansonlee Cesar")],
                        ),
                    ],
                ),
                p(
                    vec![],
                    vec![
                        text("Part of "),
                        a(
                            vec![href("http://todomvc.com")],
                            vec![text("TodoMVC")],
                        ),
                    ],
                ),
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
impl Visibility {
    fn to_uri(&self) -> String {
        match self {
            Visibility::All => "#/".to_string(),
            Visibility::Active => "#/active".to_string(),
            Visibility::Completed => "#/completed".to_string(),
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

impl Component<Msg> for Model {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Add => {
                if !self.field.is_empty() {
                    self.entries.push(Entry::new(&self.field, self.uid));
                    self.uid += 1;
                    self.field = "".into();
                }
                Cmd::none()
            }
            Msg::UpdateField(field) => {
                self.field = field;
                Cmd::none()
            }
            Msg::EditingEntry(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = true;
                    } else {
                        entry.editing = false;
                    }
                });
                Cmd::none()
            }
            Msg::UpdateEntry(id, new_desc) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.description = new_desc.clone();
                    }
                });
                Cmd::none()
            }
            Msg::Delete(id) => {
                self.entries.retain(|entry| entry.id != id);
                Cmd::none()
            }
            Msg::DeleteComplete => {
                self.entries.retain(|entry| !entry.completed);
                Cmd::none()
            }
            Msg::Check(id, is_completed) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.completed = is_completed;
                    }
                });
                Cmd::none()
            }
            Msg::CheckAll(is_completed) => {
                self.entries
                    .iter_mut()
                    .for_each(|entry| entry.completed = is_completed);
                Cmd::none()
            }
            Msg::ChangeVisibility(visibility) => {
                self.visibility = visibility;
                Cmd::none()
            }
        }
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
