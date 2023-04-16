#![deny(warnings)]
use crate::mt_dom::TreePath;
use sauron::prelude::*;

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

/// this is an inefficient patch since the blank lines are matches
/// causing the next good match to skip due to the there has been
///  a previous match with bigger node_idx
///  The solution is therefore to not put key to elements that
///  are meant to be discarded  and can easily be construcated
#[wasm_bindgen_test]
fn test1() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let current_dom: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="4638962052468762037">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>"\n"</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 0, column: 0"</div>
          </div>
       </div>
    </div>
    );

    let target_dom: Node<()> = node!(
     <div class="app">
        <h1>"Lines"</h1>
        <div>
           <div class="grid__wrapper">
              <div class="grid grid__number_wide1">
                 <div class="grid__number__line" key="4638962052468762037">
                    <div class="grid__number">"0"</div>
                    <div class="grid__line">
                       <div>"\n"</div>
                    </div>
                 </div>
                 <div class="grid__number__line" key="623356695095054844">
                    <div class="grid__number">"1"</div>
                    <div class="grid__line">
                       <div>"C"</div>
                       <div>"J"</div>
                       <div>"K"</div>
                       <div>"\n"</div>
                    </div>
                 </div>
                 <div class="grid__number__line" key="4638962052468762037">
                    <div class="grid__number">"2"</div>
                    <div class="grid__line">
                       <div>"\n"</div>
                    </div>
                 </div>
                 <div class="grid__number__line" key="9824372840226575955">
                    <div class="grid__number">"3"</div>
                    <div class="grid__line">
                       <div>"T"</div>
                       <div>"h"</div>
                       <div>"e"</div>
                       <div>"\n"</div>
                    </div>
                 </div>
              </div>
              <div class="grid__status">"line: 1, column: 0"</div>
           </div>
        </div>
     </div>
    );

    let patch = diff(&current_dom, &target_dom);

    log::trace!("test update here..");
    log::debug!("patches: {:#?}", patch);

    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            Patch::replace_node(None, TreePath::new(vec![1, 0, 0, 1, 0, 0,]), &text("0")),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1, 0, 0, 0,]),),
            Patch::replace_node(None, TreePath::new(vec![1, 0, 0, 2, 0, 0,]), &text("3")),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1, 0, 0, 2,]),
                vec![
                    &div(
                        vec![class("grid__number__line"), key("623356695095054844")],
                        vec![
                            div(vec![class("grid__number")], vec![text(1)]),
                            div(
                                vec![class("grid__line")],
                                vec![
                                    div(vec![], vec![text("C")]),
                                    div(vec![], vec![text("J")]),
                                    div(vec![], vec![text("K")]),
                                    div(vec![], vec![text("\n")]),
                                ]
                            ),
                        ]
                    ),
                    &div(
                        vec![class("grid__number__line"), key("4638962052468762037")],
                        vec![
                            div(vec![class("grid__number")], vec![text(2)]),
                            div(
                                vec![class("grid__line")],
                                vec![div(vec![], vec![text("\n")]),]
                            ),
                        ]
                    )
                ]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 1, 0,]),
                &text("line: 1, column: 0")
            ),
        ]
    );

    let simple_program = simple_program();

    simple_program.set_current_dom(current_dom.clone());

    let target_dom_html = target_dom.render_to_string();

    simple_program
        .update_dom_with_vdom(target_dom)
        .expect("must not error");

    let app_node = crate::document()
        .query_selector(".app")
        .expect("must not error")
        .expect("must exist");

    assert_eq!(target_dom_html, app_node.outer_html());
}
