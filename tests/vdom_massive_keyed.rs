use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn key_inserted_at_start() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "XXX")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::insert_before_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element(
                "div",
                vec![attr("key", "XXX")],
                vec![leaf("lineXXX")]
            )]
        )]
    );
}

#[test]
fn key_inserted_at_middle() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "XXX")], vec![leaf("lineXXX")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_after_node(
            Some(&"div"),
            TreePath::new(vec![4]),
            vec![&element(
                "div",
                vec![attr("key", "XXX")],
                vec![leaf("lineXXX")]
            )]
        )]
    );
}

#[test]
fn wrapped_elements() {
    let old: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![leaf("line1")]),
                element("div", vec![attr("key", "2")], vec![leaf("line2")]),
                element("div", vec![attr("key", "3")], vec![leaf("line3")]),
                element("div", vec![attr("key", "4")], vec![leaf("line4")]),
                element("div", vec![attr("key", "5")], vec![leaf("line5")]),
                element("div", vec![attr("key", "6")], vec![leaf("line6")]),
                element("div", vec![attr("key", "7")], vec![leaf("line7")]),
                element("div", vec![attr("key", "8")], vec![leaf("line8")]),
                element("div", vec![attr("key", "9")], vec![leaf("line9")]),
            ],
        )],
    );

    let new: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![leaf("line1")]),
                element("div", vec![attr("key", "2")], vec![leaf("line2")]),
                element("div", vec![attr("key", "3")], vec![leaf("line3")]),
                element("div", vec![attr("key", "4")], vec![leaf("line4")]),
                element("div", vec![attr("key", "5")], vec![leaf("line5")]),
                element("div", vec![attr("key", "XXX")], vec![leaf("lineXXX")]),
                element("div", vec![attr("key", "6")], vec![leaf("line6")]),
                element("div", vec![attr("key", "7")], vec![leaf("line7")]),
                element("div", vec![attr("key", "8")], vec![leaf("line8")]),
                element("div", vec![attr("key", "9")], vec![leaf("line9")]),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::insert_after_node(
            Some(&"div"),
            TreePath::new(vec![0, 4]),
            vec![&element(
                "div",
                vec![attr("key", "XXX")],
                vec![leaf("lineXXX")]
            )]
        )]
    );
}

#[test]
fn text_changed() {
    let old: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![leaf("line1")]),
                element("div", vec![attr("key", "2")], vec![leaf("line2")]),
                element("div", vec![attr("key", "3")], vec![leaf("line3")]),
                element("div", vec![attr("key", "4")], vec![leaf("line4")]),
                element("div", vec![attr("key", "5")], vec![leaf("line5")]),
                element("div", vec![attr("key", "6")], vec![leaf("line6")]),
                element("div", vec![attr("key", "7")], vec![leaf("line7")]),
                element("div", vec![attr("key", "8")], vec![leaf("line8")]),
                element("div", vec![attr("key", "9")], vec![leaf("line9")]),
            ],
        )],
    );

    let new: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![leaf("line1")]),
                element("div", vec![attr("key", "2")], vec![leaf("line2")]),
                element("div", vec![attr("key", "3")], vec![leaf("line3")]),
                element("div", vec![attr("key", "4")], vec![leaf("line4")]),
                element("div", vec![attr("key", "5")], vec![leaf("line5")]),
                element("div", vec![attr("key", "6")], vec![leaf("line6")]),
                element("div", vec![attr("key", "7")], vec![leaf("line7_changed")]),
                element("div", vec![attr("key", "8")], vec![leaf("line8")]),
                element("div", vec![attr("key", "9")], vec![leaf("line9")]),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            None,
            TreePath::new(vec![0, 6, 0]),
            vec![&leaf("line7_changed")]
        )]
    );
}

#[test]
fn text_changed_non_keyed() {
    let old: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![], vec![leaf("line1")]),
                element("div", vec![], vec![leaf("line2")]),
                element("div", vec![], vec![leaf("line3")]),
                element("div", vec![], vec![leaf("line4")]),
                element("div", vec![], vec![leaf("line5")]),
                element("div", vec![], vec![leaf("line6")]),
                element("div", vec![], vec![leaf("line7")]),
                element("div", vec![], vec![leaf("line8")]),
                element("div", vec![], vec![leaf("line9")]),
            ],
        )],
    );

    let new: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![], vec![leaf("line1")]),
                element("div", vec![], vec![leaf("line2")]),
                element("div", vec![], vec![leaf("line3")]),
                element("div", vec![], vec![leaf("line4")]),
                element("div", vec![], vec![leaf("line5")]),
                element("div", vec![], vec![leaf("line6")]),
                element("div", vec![], vec![leaf("line7_changed")]),
                element("div", vec![], vec![leaf("line8")]),
                element("div", vec![], vec![leaf("line9")]),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::replace_node(
            None,
            TreePath::new(vec![0, 6, 0]),
            vec![&leaf("line7_changed")]
        )]
    );
}

#[test]
fn insert_one_line_at_start() {
    let old: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![leaf("1")]),
                        element("div", vec![], vec![leaf("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![leaf("2")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![leaf("3")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
            ],
        )],
    );

    let new: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![leaf("1")]),
                        element("div", vec![], vec![leaf("XXX")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![leaf("2")]),
                        element("div", vec![], vec![leaf("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![leaf("3")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![leaf("4")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::replace_node(None, TreePath::new(vec![0, 2, 0, 0]), vec![&leaf("4")]),
            Patch::replace_node(None, TreePath::new(vec![0, 1, 0, 0]), vec![&leaf("3")]),
            Patch::replace_node(None, TreePath::new(vec![0, 0, 0, 0]), vec![&leaf("2")]),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![0, 0]),
                vec![&element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![leaf("1")]),
                        element("div", vec![], vec![leaf("XXX")]),
                    ],
                )],
            ),
        ]
    );
}

#[test]
fn insert_two_lines_at_start() {
    let old: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![leaf("1")]),
                        element("div", vec![], vec![leaf("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![leaf("2")]),
                        element("div", vec![], vec![leaf("line2")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![leaf("2")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
            ],
        )],
    );

    let new: Node<()> = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![leaf("1")]),
                        element("div", vec![], vec![leaf("XXX")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hashYYY")],
                    vec![
                        element("div", vec![], vec![leaf("2")]),
                        element("div", vec![], vec![leaf("YYY")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![leaf("3")]),
                        element("div", vec![], vec![leaf("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![leaf("4")]),
                        element("div", vec![], vec![leaf("line2")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![leaf("5")]),
                        element("div", vec![], vec![leaf("line3")]),
                    ],
                ),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::replace_node(None, TreePath::new(vec![0, 2, 0, 0]), vec![&leaf("5")]),
            Patch::replace_node(None, TreePath::new(vec![0, 1, 0, 0]), vec![&leaf("4")]),
            Patch::replace_node(None, TreePath::new(vec![0, 0, 0, 0]), vec![&leaf("3")]),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![0, 0]),
                vec![
                    &element(
                        "div",
                        vec![attr("key", "hashXXX")],
                        vec![
                            element("div", vec![], vec![leaf("1")]),
                            element("div", vec![], vec![leaf("XXX")]),
                        ],
                    ),
                    &element(
                        "div",
                        vec![attr("key", "hashYYY")],
                        vec![
                            element("div", vec![], vec![leaf("2")]),
                            element("div", vec![], vec![leaf("YYY")]),
                        ],
                    )
                ]
            ),
        ]
    );
}
