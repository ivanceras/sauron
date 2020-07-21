use sauron::{
    dom::events::KeyboardEvent,
    html::{attributes::*, *},
    prelude::*,
    Cmd, Component, Node,
};

pub struct Model {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl Model {
    pub fn new() -> Self {
        Model {
            entries: vec![],
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        }
    }
}

impl Component<Msg> for Model {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.entries.push(entry);
                self.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.edit_value.clone();
                self.complete_edit(idx, edit_value);
                self.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::UpdateEdit(val) => {
                self.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.edit_value = self.entries[idx].description.clone();
                self.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.is_all_completed();
                self.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.clear_completed();
            }
            Msg::Nope => {}
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("todomvc-wrapper")],
            vec![
                section(
                    vec![class("todoapp")],
                    vec![
                        header(
                            vec![class("header")],
                            vec![
                                h1(vec![], vec![text("todos")]),
                                self.view_input(),
                            ],
                        ),
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
                                        .filter(|e| self.filter.fit(e))
                                        .enumerate()
                                        .map(view_entry)
                                        .collect::<Vec<Node<Msg>>>()
                                }),
                            ],
                        ),
                        footer(
                            vec![class("footer")],
                            vec![
                                span(
                                    vec![class("todo-count")],
                                    vec![
                                        strong(
                                            vec![],
                                            vec![text(format!(
                                                "{}",
                                                self.total()
                                            ))],
                                        ),
                                        text(" item(s) left"),
                                    ],
                                ),
                                ul(
                                    vec![class("filters")],
                                    vec![
                                        self.view_filter(Filter::All),
                                        self.view_filter(Filter::Active),
                                        self.view_filter(Filter::Completed),
                                    ],
                                ),
                                button(
                                    vec![
                                        class("clear-completed"),
                                        on_click(|_| Msg::ClearCompleted),
                                    ],
                                    vec![text(format!(
                                        "Clear completed ({})",
                                        self.total_completed()
                                    ))],
                                ),
                            ],
                        ),
                    ],
                ),
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
                                    vec![
                                        href("http://todomvc.com/"),
                                        target("_blank"),
                                    ],
                                    vec![text("TodoMVC")],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        )
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Node<Msg> {
        let flt = filter.clone();
        li(
            vec![],
            vec![a(
                vec![
                    class(if self.filter == flt {
                        "selected"
                    } else {
                        "not-selected"
                    }),
                    href(flt.to_string()),
                    on_click(move |_| Msg::SetFilter(flt.clone())),
                ],
                vec![text(filter.to_string())],
            )],
        )
    }

    fn view_input(&self) -> Node<Msg> {
        input(
            vec![
                class("new-todo"),
                id("new-todo"),
                placeholder("What needs to be done?"),
                value(self.value.to_string()),
                on_input(|v: InputEvent| Msg::Update(v.value.to_string())),
                on_keypress(|event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        Msg::Add
                    } else {
                        Msg::Nope
                    }
                }),
            ],
            vec![],
        )
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Node<Msg> {
    li(
        vec![classes_flag([
            ("todo", true),
            ("editing", entry.editing),
            ("completed", entry.completed),
        ])],
        vec![
            div(
                vec![class("view")],
                vec![
                    input(
                        vec![
                            class("toggle"),
                            r#type("checkbox"),
                            checked(entry.completed),
                            on_click(move |_| Msg::Toggle(idx)),
                        ],
                        vec![],
                    ),
                    label(
                        vec![on_doubleclick(move |_| Msg::ToggleEdit(idx))],
                        vec![text(format!("{}", entry.description))],
                    ),
                    button(
                        vec![
                            class("destroy"),
                            on_click(move |_| Msg::Remove(idx)),
                        ],
                        vec![],
                    ),
                ],
            ),
            { view_entry_edit_input((idx, &entry)) },
        ],
    )
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Node<Msg> {
    if entry.editing {
        input(
            vec![
                class("edit"),
                r#type("text"),
                value(&entry.description),
                on_input(|input: InputEvent| {
                    Msg::UpdateEdit(input.value.to_string())
                }),
                on_blur(move |_| Msg::Edit(idx)),
                on_keypress(move |event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        Msg::Edit(idx)
                    } else {
                        Msg::Nope
                    }
                }),
            ],
            vec![],
        )
    } else {
        input(vec![r#type("hidden")], vec![])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match *self {
            Filter::All => "#/".to_string(),
            Filter::Active => "#/active".to_string(),
            Filter::Completed => "#/completed".to_string(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl Model {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}
