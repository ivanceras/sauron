use sauron::{
    html::{attributes::*, *},
    *,
};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

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
    console_log::init_with_level(log::Level::Trace).ok();
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

    let patches1 = diff(&old, &update1).unwrap();

    log::trace!("patches1: {:#?} at line: {}", patches1, line!());

    let old_html = old.render_to_string();
    log::trace!("old html: {}", old_html);
    let expected_old = "<main class=\"editor\">\
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

    assert_eq!(old_html, expected_old);

    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(old.clone())
        .expect("must not error");

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

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

    assert_eq!(old.render_to_string(), expected);

    assert_eq!(expected, container.outer_html());

    simple_program
        .update_dom_with_vdom(update1.clone())
        .expect("must not error");

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

    log::trace!("expected1 {:?}", container.outer_html());

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

    assert_eq!(expected1, update1.render_to_string());
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

    {
        let root_node = simple_program.root_node.borrow();
        let root_node_html = root_node.as_ref().unwrap().outer_html();
        log::trace!("current root node: {}", root_node_html);
    }
    let patches2 = diff(&update1, &update2).unwrap();
    log::trace!("-->patches2: {:#?}", patches2);

    log::info!("Updating dom with update2");
    simple_program
        .update_dom_with_vdom(update2.clone())
        .expect("must not error");
    log::info!("Done doing updates2");

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

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

    assert_eq!(expected2, update2.render_to_string());
    assert_eq!(expected2, container.outer_html());

    let update3: Node<()> = main(
        vec![class("editor")],
        vec![
            section(
                vec![class("lines")],
                vec![
                    div(
                        vec![key("hashZZZ")],
                        vec![div(vec![], vec![text("0")]), div(vec![], vec![text("\n")])],
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

    let patches3 = diff(&update2, &update3).unwrap();
    log::trace!("\n---->patches3: {:#?}", patches3);

    simple_program
        .update_dom_with_vdom(update3.clone())
        .expect("must not error");

    let container = document
        .query_selector(".editor")
        .expect("must not error")
        .expect("must exist");

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

    assert_eq!(expected3, update3.render_to_string());
    assert_eq!(expected3, container.outer_html());
}
