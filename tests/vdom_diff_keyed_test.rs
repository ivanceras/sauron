use sauron::vdom::{diff::*, patch::*, *};

#[test]
fn keyed_no_changed() {
    let old: Node<()> = element(
        "div",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![]);
}

#[test]
fn key_1_removed_at_start() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![0]))]
    );
}

#[test]
fn non_unique_keys_matched_at_old() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![1]))]
    );
}

#[test]
fn key_2_removed_at_the_end() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![1]),)]
    );
}

#[test]
fn key_2_removed_at_the_middle() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let diff = diff(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![1]))]
    );
}

//TODO: currently can not deal with repeated keys
//#[test]
fn there_are_2_exact_same_keys_in_the_old() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("0")]),
            element("div", vec![attr("key", "1")], vec![leaf("1")]),
            element("div", vec![attr("key", "3")], vec![leaf("2")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("1")]),
            element("div", vec![attr("key", "3")], vec![leaf("2")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 0]),
                vec![&leaf("1")]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1]))
        ]
    );
}

//TODO: currently can not deal with repeated keys
#[test]
fn there_are_2_exact_same_keys_in_the_new() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("0")]),
            element("div", vec![attr("key", "3")], vec![leaf("2")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("1")]),
            element("div", vec![attr("key", "1")], vec![leaf("1")]),
            element("div", vec![attr("key", "3")], vec![leaf("2")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::replace_node(None, TreePath::new([0, 0]), vec![&leaf("1")]),
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new([0]),
                vec![&element("div", vec![attr("key", "1")], vec![leaf("1")])]
            ),
        ]
    );
}

//TODO: currently can not deal with repeated keys
//#[test]
fn there_are_2_exact_same_keys_in_both_old_and_new() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("0")]), //matched 1
            element("div", vec![attr("key", "3")], vec![leaf("1")]),
            element("div", vec![attr("key", "3")], vec![leaf("2")]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("1")]), //matched 1
            element("div", vec![attr("key", "1")], vec![leaf("2")]),
            element("div", vec![attr("key", "3")], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 0]),
                vec![&leaf("1")]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0]),
                vec![&leaf("3")]
            ),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&element("div", vec![attr("key", "1")], vec![leaf("2")])]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![2])),
        ]
    );
}

#[test]
fn key_2_inserted_at_start() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "1")], vec![]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_before_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
    );
}

#[test]
fn keyed_element_not_reused() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
    );
}

// altered to work with diff using lis
#[test]
fn key_2_inserted_at_the_end() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::append_children(
            Some(&"main"),
            TreePath::new(vec![]),
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
    );
}

#[test]
fn test_append_at_sub_level() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![element("div", vec![attr("key", "1")], vec![leaf("1")])],
        )],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![
                element("div", vec![attr("key", "1")], vec![leaf("1")]),
                element("div", vec![attr("key", "2")], vec![leaf("2")]),
                element("div", vec![attr("key", "3")], vec![leaf("3")]),
            ],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::append_children(
            Some(&"main"),
            TreePath::new(vec![0]),
            vec![
                &element("div", vec![attr("key", "2")], vec![leaf("2")]),
                &element("div", vec![attr("key", "3")], vec![leaf("3")])
            ],
        )]
    )
}

#[test]
fn key_2_inserted_in_the_middle() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let diff = diff(&old, &new);

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_after_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
    );
}

#[test]
fn key1_removed_at_start_then_key2_has_additional_attributes() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "div",
            vec![attr("key", "2"), attr("class", "some-class")],
            vec![],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    // we add attrubutes at node index 2, and this will become a node index 1
    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new(vec![1]),
                vec![&attr("class", "some-class")]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0]),),
        ]
    );
}

#[test]
fn deep_nested_key1_removed_at_start_then_key2_has_additional_attributes() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "1")], vec![]),
                element("div", vec![attr("key", "2")], vec![]),
            ],
        )],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![],
            )],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new(vec![0, 1]),
                vec![&attr("class", "some-class")]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0, 0]),),
        ]
    );
}

#[test]
fn deep_nested_more_children_key0_and_key1_removed_at_start_then_key2_has_additional_attributes(
) {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "0")], vec![]),
                element("div", vec![attr("key", "1")], vec![]),
                element("div", vec![attr("key", "2")], vec![]),
            ],
        )],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![],
            )],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new(vec![0, 2]),
                vec![&attr("class", "some-class")]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0, 0]),),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0, 1]),),
        ]
    );
}

#[test]
fn deep_nested_keyed_with_non_keyed_children() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "0")], vec![]),
                element("div", vec![attr("key", "1")], vec![]),
                element(
                    "div",
                    vec![attr("key", "2")],
                    vec![
                        element("p", vec![], vec![leaf("paragraph1")]),
                        element(
                            "a",
                            vec![attr("href", "#link1")],
                            vec![leaf("Click here")],
                        ),
                    ],
                ),
            ],
        )],
    );

    let new: Node<()> = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![
                    element(
                        "p",
                        vec![],
                        vec![leaf("paragraph1, with added content")],
                    ),
                    element(
                        "a",
                        vec![attr("href", "#link1")],
                        vec![leaf("Click here to continue")],
                    ),
                ],
            )],
        )],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new(vec![0, 2]),
                vec![&attr("class", "some-class")]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 2, 0, 0]),
                vec![&leaf("paragraph1, with added content")]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 2, 1, 0]),
                vec![&leaf("Click here to continue")]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0, 0]),),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0, 1]),),
        ]
    );
}

#[test]
fn text_changed_in_keyed_elements() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![element(
            "section",
            vec![attr("class", "todo")],
            vec![
                element("article", vec![attr("key", "1")], vec![leaf("item1")]),
                element("article", vec![attr("key", "2")], vec![leaf("item2")]),
                element("article", vec![attr("key", "3")], vec![leaf("item3")]),
            ],
        )],
    );

    // we remove the key1, and change the text in item3
    let update1: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![element(
            "section",
            vec![attr("class", "todo")],
            vec![
                element("article", vec![attr("key", "2")], vec![leaf("item2")]),
                element(
                    "article",
                    vec![attr("key", "3")],
                    vec![leaf("item3 with changes")],
                ),
            ],
        )],
    );

    let patch = diff(&old, &update1);
    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 2, 0]),
                vec![&leaf("item3 with changes")]
            ),
            Patch::remove_node(Some(&"article"), TreePath::new(vec![0, 0])),
        ]
    );
}

#[test]
fn text_changed_in_mixed_keyed_and_non_keyed_elements() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "1")],
                        vec![leaf("item1")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![leaf("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![leaf("item3")],
                    ),
                ],
            ),
            element("footer", vec![], vec![leaf("3 items left")]),
        ],
    );

    // we remove the key1, and change the text in item3
    let update1: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![leaf("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![leaf("item3 with changes")],
                    ),
                ],
            ),
            element("footer", vec![], vec![leaf("2 items left")]),
        ],
    );

    let patch = diff(&old, &update1);
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![0, 2, 0]),
                vec![&leaf("item3 with changes")]
            ),
            Patch::remove_node(Some(&"article"), TreePath::new(vec![0, 0]),),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0]),
                vec![&leaf("2 items left")]
            ),
        ]
    );
}

/// mixed of keyed and non-keyed elements
#[test]
fn test12() {
    let old: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![leaf("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "1")],
                        vec![leaf("item1")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![leaf("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![leaf("item3")],
                    ),
                ],
            ),
            element("footer", vec![], vec![leaf("3 items left")]),
        ],
    );

    // we remove the key1, and change the text in item3
    let update1: Node<()> = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![leaf("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![leaf("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![leaf("item3 with changes")],
                    ),
                ],
            ),
            element("footer", vec![], vec![leaf("2 items left")]),
        ],
    );

    let patch = diff(&old, &update1);
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 2, 0]),
                vec![&leaf("item3 with changes")]
            ),
            Patch::remove_node(Some(&"article"), TreePath::new(vec![1, 0]),),
            Patch::replace_node(
                None,
                TreePath::new(vec![2, 0]),
                vec![&leaf("2 items left")]
            ),
        ]
    );
}

#[test]
fn remove_first() {
    let old: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "3")], vec![leaf("3")]),
        ],
    );

    let new: Node<()> = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "3")], vec![leaf("3")]),
        ],
    );

    let diff = diff(&old, &new);
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![0]),)]
    )
}
