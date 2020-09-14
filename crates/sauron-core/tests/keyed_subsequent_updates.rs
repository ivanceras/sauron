use log::*;
use sauron_core::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    mt_dom::patch::*,
    Patch,
    *,
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;
use web_sys::InputEvent;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

// Issue: When there is diff_keyed_elements
// the first update is OK, however, the subsequent update
// will error with:
//
// : panicked at 'must have a tag here',
// sauron/crates/sauron-core/src/dom/apply_patches.rs:109:32

#[wasm_bindgen_test]
fn subsequent_updates() {
    console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("editor")],
        vec![
            section(
                vec![class("lines")],
                vec![
                    div(
                        vec![key("hash0")],
                        vec![
                            div(vec![], vec![text("0")]),
                            div(vec![], vec![text("line0")]),
                        ],
                    ),
                    div(
                        vec![key("hash1")],
                        vec![
                            div(vec![], vec![text("1")]),
                            div(vec![], vec![text("line1")]),
                        ],
                    ),
                    div(
                        vec![key("hash2")],
                        vec![
                            div(vec![], vec![text("2")]),
                            div(vec![], vec![text("line2")]),
                        ],
                    ),
                    div(
                        vec![key("hash3")],
                        vec![
                            div(vec![], vec![text("3")]),
                            div(vec![], vec![text("line3")]),
                        ],
                    ),
                ],
            ),
            footer(vec![], vec![text("line:0, col:0")]),
        ],
    );

    let update1: Node<()> = main(
        vec![class("editor")],
        vec![
            section(
                vec![class("lines")],
                vec![
                    div(
                        vec![key("hashXXX")],
                        vec![
                            div(vec![], vec![text("0")]),
                            div(vec![], vec![text("lineXXX")]),
                        ],
                    ),
                    div(
                        vec![key("hash0")],
                        vec![
                            div(vec![], vec![text("1")]),
                            div(vec![], vec![text("line0")]),
                        ],
                    ),
                    div(
                        vec![key("hash1")],
                        vec![
                            div(vec![], vec![text("2")]),
                            div(vec![], vec![text("line1")]),
                        ],
                    ),
                    div(
                        vec![key("hash2")],
                        vec![
                            div(vec![], vec![text("3")]),
                            div(vec![], vec![text("line2")]),
                        ],
                    ),
                    div(
                        vec![key("hash3")],
                        vec![
                            div(vec![], vec![text("4")]),
                            div(vec![], vec![text("line3")]),
                        ],
                    ),
                ],
            ),
            footer(vec![], vec![text("line:0, col:0")]),
        ],
    );

    let patches1 = diff(&old, &update1);

    log::trace!("patches1: {:#?}", patches1);
    assert_eq!(
        patches1,
        vec![
            ChangeText::new(4, "0", "1").into(),
            ChangeText::new(9, "1", "2").into(),
            ChangeText::new(14, "2", "3").into(),
            ChangeText::new(19, "3", "4").into(),
            InsertChildren::new(
                &"section",
                1,
                0,
                vec![&div(
                    vec![key("hashXXX")],
                    vec![
                        div(vec![], vec![text("0")]),
                        div(vec![], vec![text("lineXXX")]),
                    ],
                )]
            )
            .into()
        ]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");
    log::trace!("old html: {}", old_html);
    #[cfg(not(feature = "with-measure"))]
    let expected_old = r#"<main class="editor">
    <section class="lines">
        <div key="hash0">
            <div>0</div>
            <div>line0</div>
        </div>
        <div key="hash1">
            <div>1</div>
            <div>line1</div>
        </div>
        <div key="hash2">
            <div>2</div>
            <div>line2</div>
        </div>
        <div key="hash3">
            <div>3</div>
            <div>line3</div>
        </div>
    </section>
    <footer>line:0, col:0</footer>
</main>"#;

    #[cfg(feature = "with-measure")]
    let expected_old = r#"<main class="editor" node_idx="0">
    <section class="lines" node_idx="1">
        <div key="hash0" node_idx="2">
            <div node_idx="3">0</div>
            <div node_idx="5">line0</div>
        </div>
        <div key="hash1" node_idx="7">
            <div node_idx="8">1</div>
            <div node_idx="10">line1</div>
        </div>
        <div key="hash2" node_idx="12">
            <div node_idx="13">2</div>
            <div node_idx="15">line2</div>
        </div>
        <div key="hash3" node_idx="17">
            <div node_idx="18">3</div>
            <div node_idx="20">line3</div>
        </div>
    </section>
    <footer node_idx="22">line:0, col:0</footer>
</main>"#;
    assert_eq!(old_html, expected_old);

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old.clone(),
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

    #[cfg(not(feature = "with-measure"))]
    let expected = "<main class=\"editor\">\
                        <section class=\"lines\">\
                            <div key=\"hash0\">\
                                <div>0</div>\
                                <div>line0</div>\
                            </div>\
                            <div key=\"hash1\">\
                                <div>1</div>\
                                <div>line1</div>\
                            </div>\
                            <div key=\"hash2\">\
                                <div>2</div>\
                                <div>line2</div>\
                            </div>\
                            <div key=\"hash3\">\
                                <div>3</div>\
                                <div>line3</div>\
                            </div>\
                        </section>\
                            <footer>line:0, col:0</footer>\
                        </main>";

    let mut old_render = String::new();
    old.render_compressed(&mut old_render).expect("must render");
    assert_eq!(old_render, expected);

    #[cfg(feature = "with-measure")]
    let expected = "<main class=\"editor\" node_idx=\"0\">\
                        <section class=\"lines\" node_idx=\"1\">\
                            <div key=\"hash0\" node_idx=\"2\">\
                                <div node_idx=\"3\">0</div>\
                                <div node_idx=\"5\">line0</div>\
                            </div>\
                            <div key=\"hash1\" node_idx=\"7\">\
                                <div node_idx=\"8\">1</div>\
                                <div node_idx=\"10\">line1</div>\
                            </div>\
                            <div key=\"hash2\" node_idx=\"12\">\
                                <div node_idx=\"13\">2</div>\
                                <div node_idx=\"15\">line2</div>\
                            </div>\
                            <div key=\"hash3\" node_idx=\"17\">\
                                <div node_idx=\"18\">3</div>\
                                <div node_idx=\"20\">line3</div>\
                            </div>\
                        </section>\
                            <footer node_idx=\"22\">line:0, col:0</footer>\
                        </main>";

    log::trace!("expected: {:?}", container.outer_html());
    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1.clone());

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

    log::trace!("expected1 {:?}", container.outer_html());

    #[cfg(not(feature = "with-measure"))]
    let expected1 = "<main class=\"editor\">\
                        <section class=\"lines\">\
                            <div key=\"hashXXX\">\
                                <div>0</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\">\
                                <div>1</div>\
                                <div>line0</div>\
                            </div>\
                            <div key=\"hash1\">\
                                <div>2</div>\
                                <div>line1</div>\
                            </div>\
                            <div key=\"hash2\">\
                                <div>3</div>\
                                <div>line2</div>\
                            </div>\
                            <div key=\"hash3\">\
                                <div>4</div>\
                                <div>line3</div>\
                            </div>\
                        </section>\
                        <footer>line:0, col:0</footer>\
                        </main>";

    // The node_idx here is from the previous DOM, and since
    // node_idx attribute is not diff therefore there is no patch for it.
    #[cfg(feature = "with-measure")]
    let expected1 = "<main class=\"editor\" node_idx=\"0\">\
                        <section class=\"lines\" node_idx=\"1\">\
                            <div key=\"hashXXX\">\
                                <div>0</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\" node_idx=\"2\">\
                                <div node_idx=\"3\">1</div>\
                                <div node_idx=\"5\">line0</div>\
                            </div>\
                            <div key=\"hash1\" node_idx=\"7\">\
                                <div node_idx=\"8\">2</div>\
                                <div node_idx=\"10\">line1</div>\
                            </div>\
                            <div key=\"hash2\" node_idx=\"12\">\
                                <div node_idx=\"13\">3</div>\
                                <div node_idx=\"15\">line2</div>\
                            </div>\
                            <div key=\"hash3\" node_idx=\"17\">\
                                <div node_idx=\"18\">4</div>\
                                <div node_idx=\"20\">line3</div>\
                            </div>\
                        </section>\
                            <footer node_idx=\"22\">line:0, col:0</footer>\
                        </main>";

    let mut update1_render = String::new();
    update1
        .render_compressed(&mut update1_render)
        .expect("must render");
    assert_eq!(expected1, update1_render);
    assert_eq!(expected1, container.outer_html());

    let update2: Node<()> = main(
        vec![class("editor")],
        vec![
            section(
                vec![class("lines")],
                vec![
                    div(
                        vec![key("hashYYY")],
                        vec![
                            div(vec![], vec![text("0")]),
                            div(vec![], vec![text("lineYYY")]),
                        ],
                    ),
                    div(
                        vec![key("hashXXX")],
                        vec![
                            div(vec![], vec![text("1")]),
                            div(vec![], vec![text("lineXXX")]),
                        ],
                    ),
                    div(
                        vec![key("hash0")],
                        vec![
                            div(vec![], vec![text("2")]),
                            div(vec![], vec![text("line0")]),
                        ],
                    ),
                    div(
                        vec![key("hash1")],
                        vec![
                            div(vec![], vec![text("3")]),
                            div(vec![], vec![text("line1")]),
                        ],
                    ),
                    div(
                        vec![key("hash2")],
                        vec![
                            div(vec![], vec![text("4")]),
                            div(vec![], vec![text("line2")]),
                        ],
                    ),
                    div(
                        vec![key("hash3")],
                        vec![
                            div(vec![], vec![text("5")]),
                            div(vec![], vec![text("line3")]),
                        ],
                    ),
                ],
            ),
            footer(vec![], vec![text("line:0, col:0")]),
        ],
    );

    let patches2 = diff(&update1, &update2);
    log::trace!("patches2: {:#?}", patches2);
    assert_eq!(
        patches2,
        vec![
            ChangeText::new(4, "0", "1").into(),
            ChangeText::new(9, "1", "2").into(),
            ChangeText::new(14, "2", "3").into(),
            ChangeText::new(19, "3", "4").into(),
            ChangeText::new(24, "4", "5").into(),
            InsertChildren::new(
                &"section",
                1,
                0,
                vec![&div(
                    vec![key("hashYYY")],
                    vec![
                        div(vec![], vec![text("0")]),
                        div(vec![], vec![text("lineYYY")]),
                    ],
                ),]
            )
            .into()
        ]
    );

    dom_updater.update_dom(&simple_program, update2.clone());

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

    #[cfg(not(feature = "with-measure"))]
    let expected2 = "<main class=\"editor\">\
                        <section class=\"lines\">\
                            <div key=\"hashYYY\">\
                                <div>0</div>\
                                <div>lineYYY</div>\
                            </div>\
                            <div key=\"hashXXX\">\
                                <div>1</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\">\
                                <div>2</div>\
                                <div>line0</div>\
                            </div>\
                            <div key=\"hash1\">\
                                <div>3</div>\
                                <div>line1</div>\
                            </div>\
                            <div key=\"hash2\">\
                                <div>4</div>\
                                <div>line2</div>\
                            </div>\
                            <div key=\"hash3\">\
                                <div>5</div>\
                                <div>line3</div>\
                            </div>\
                        </section>\
                            <footer>line:0, col:0</footer>\
                        </main>";

    #[cfg(feature = "with-measure")]
    let expected2 = "<main class=\"editor\" node_idx=\"0\">\
                        <section class=\"lines\" node_idx=\"1\">\
                            <div key=\"hashYYY\">\
                                <div>0</div>\
                                <div>lineYYY</div>\
                            </div>\
                            <div key=\"hashXXX\">\
                                <div>1</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\" node_idx=\"2\">\
                                <div node_idx=\"3\">2</div>\
                                <div node_idx=\"5\">line0</div>\
                            </div>\
                            <div key=\"hash1\" node_idx=\"7\">\
                                <div node_idx=\"8\">3</div>\
                                <div node_idx=\"10\">line1</div>\
                            </div>\
                            <div key=\"hash2\" node_idx=\"12\">\
                                <div node_idx=\"13\">4</div>\
                                <div node_idx=\"15\">line2</div>\
                            </div>\
                            <div key=\"hash3\" node_idx=\"17\">\
                                <div node_idx=\"18\">5</div>\
                                <div node_idx=\"20\">line3</div>\
                            </div>\
                        </section>\
                            <footer node_idx=\"22\">line:0, col:0</footer>\
                        </main>";

    let mut update2_render = String::new();
    update2
        .render_compressed(&mut update2_render)
        .expect("must render");
    assert_eq!(expected2, update2_render);
    assert_eq!(expected2, container.outer_html());

    let update3: Node<()> = main(
        vec![class("editor")],
        vec![
            section(
                vec![class("lines")],
                vec![
                    div(
                        vec![key("hashZZZ")],
                        vec![
                            div(vec![], vec![text("0")]),
                            div(vec![], vec![text("\n")]),
                        ],
                    ),
                    div(
                        vec![key("hashYYY")],
                        vec![
                            div(vec![], vec![text("1")]),
                            div(vec![], vec![text("lineYYY")]),
                        ],
                    ),
                    div(
                        vec![key("hashXXX")],
                        vec![
                            div(vec![], vec![text("2")]),
                            div(vec![], vec![text("lineXXX")]),
                        ],
                    ),
                    div(
                        vec![key("hash0")],
                        vec![
                            div(vec![], vec![text("3")]),
                            div(vec![], vec![text("line0")]),
                        ],
                    ),
                    div(
                        vec![key("hash1")],
                        vec![
                            div(vec![], vec![text("4")]),
                            div(vec![], vec![text("line1")]),
                        ],
                    ),
                    div(
                        vec![key("hash2")],
                        vec![
                            div(vec![], vec![text("5")]),
                            div(vec![], vec![text("line2")]),
                        ],
                    ),
                    div(
                        vec![key("hash3")],
                        vec![
                            div(vec![], vec![text("6")]),
                            div(vec![], vec![text("line3")]),
                        ],
                    ),
                ],
            ),
            footer(vec![], vec![text("line:0, col:0")]),
        ],
    );

    let patches3 = diff(&update2, &update3);
    log::trace!("patches2: {:#?}", patches3);
    assert_eq!(
        patches3,
        vec![
            ChangeText::new(4, "0", "1").into(),
            ChangeText::new(9, "1", "2").into(),
            ChangeText::new(14, "2", "3").into(),
            ChangeText::new(19, "3", "4").into(),
            ChangeText::new(24, "4", "5").into(),
            ChangeText::new(29, "5", "6").into(),
            InsertChildren::new(
                &"section",
                1,
                0,
                vec![&div(
                    vec![key("hashZZZ")],
                    vec![
                        div(vec![], vec![text("0")]),
                        div(vec![], vec![text("\n")]),
                    ],
                ),]
            )
            .into()
        ]
    );

    dom_updater.update_dom(&simple_program, update3.clone());

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

    #[cfg(not(feature = "with-measure"))]
    let expected3 = "<main class=\"editor\">\
                        <section class=\"lines\">\
                            <div key=\"hashZZZ\">\
                                <div>0</div>\
                                <div>\n</div>\
                            </div>\
                            <div key=\"hashYYY\">\
                                <div>1</div>\
                                <div>lineYYY</div>\
                            </div>\
                            <div key=\"hashXXX\">\
                                <div>2</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\">\
                                <div>3</div>\
                                <div>line0</div>\
                            </div>\
                            <div key=\"hash1\">\
                                <div>4</div>\
                                <div>line1</div>\
                            </div>\
                            <div key=\"hash2\">\
                                <div>5</div>\
                                <div>line2</div>\
                            </div>\
                            <div key=\"hash3\">\
                                <div>6</div>\
                                <div>line3</div>\
                            </div>\
                        </section>\
                            <footer>line:0, col:0</footer>\
                        </main>";

    #[cfg(feature = "with-measure")]
    let expected3 = "<main class=\"editor\" node_idx=\"0\">\
                        <section class=\"lines\" node_idx=\"1\">\
                            <div key=\"hashZZZ\">\
                                <div>0</div>\
                                <div>\n</div>\
                            </div>\
                            <div key=\"hashYYY\">\
                                <div>1</div>\
                                <div>lineYYY</div>\
                            </div>\
                            <div key=\"hashXXX\">\
                                <div>2</div>\
                                <div>lineXXX</div>\
                            </div>\
                            <div key=\"hash0\" node_idx=\"2\">\
                                <div node_idx=\"3\">3</div>\
                                <div node_idx=\"5\">line0</div>\
                            </div>\
                            <div key=\"hash1\" node_idx=\"7\">\
                                <div node_idx=\"8\">4</div>\
                                <div node_idx=\"10\">line1</div>\
                            </div>\
                            <div key=\"hash2\" node_idx=\"12\">\
                                <div node_idx=\"13\">5</div>\
                                <div node_idx=\"15\">line2</div>\
                            </div>\
                            <div key=\"hash3\" node_idx=\"17\">\
                                <div node_idx=\"18\">6</div>\
                                <div node_idx=\"20\">line3</div>\
                            </div>\
                        </section>\
                            <footer node_idx=\"22\">line:0, col:0</footer>\
                        </main>";

    let mut update3_render = String::new();
    update3
        .render_compressed(&mut update3_render)
        .expect("must render");
    assert_eq!(expected3, update3_render);
    assert_eq!(expected3, container.outer_html());
}
