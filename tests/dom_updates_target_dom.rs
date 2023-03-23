#![deny(warnings)]
use crate::mt_dom::TreePath;
use sauron::prelude::*;

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn multiple_match_on_keyed_elements() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let current_dom: Node<()> = node!(
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
                <div class="grid__number__line" key="4638962052468762037">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="4638962052468762037">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>"\n"</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 2, column: 0"</div>
          </div>
       </div>
    </div>
    );

    let patches = diff(&current_dom, &target_dom);
    log::trace!("patches: {:#?}", patches);

    assert_eq!(
        patches,
        vec![
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 0, 2, 0, 0,]),
                &text("1")
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1, 0, 0, 1,]),),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 0, 3, 0, 0,]),
                &text("4")
            ),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1, 0, 0, 3,]),
                vec![
                    &node!(
                        <div class="grid__number__line" key="623356695095054844">
                           <div class="grid__number">"2"</div>
                           <div class="grid__line">
                              <div>"C"</div>
                              <div>"J"</div>
                              <div>"K"</div>
                              <div>"\n"</div>
                           </div>
                        </div>
                    ),
                    &node!(
                        <div class="grid__number__line" key="4638962052468762037">
                           <div class="grid__number">"3"</div>
                           <div class="grid__line">
                              <div>"\n"</div>
                           </div>
                        </div>
                    )
                ]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 1, 0,]),
                &text("line: 2, column: 0")
            ),
        ]
    );

    log::trace!("current_dom: {}", current_dom.render_to_string());
    log::trace!("target_dom: {}", target_dom.render_to_string());

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        current_dom.clone(),
        &sauron_core::body(),
    );

    let target_dom_html = target_dom.render_to_string();
    dom_updater.update_dom(&simple_program, target_dom).await.expect("must not error");

    let app_node = crate::document()
        .query_selector(".app")
        .expect("must not error")
        .expect("must exist");

    assert_eq!(target_dom_html, app_node.outer_html());
}

#[test]
fn unmatching_result_dom() {
    let target_html = r#"<div class=\"app\">
   <h1>Lines</h1>
   <div>
      <div class=\"grid__wrapper\">
         <div class=\"grid grid__number_wide1\">
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">0</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">1</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"623356695095054844\">
               <div class=\"grid__number\">2</div>
               <div class=\"grid__line\">
                  <div>C</div>
                  <div>J</div>
                  <div>K</div>
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">3</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"9824372840226575955\">
               <div class=\"grid__number\">4</div>
               <div class=\"grid__line\">
                  <div>T</div>
                  <div>h</div>
                  <div>e</div>
                  <div>\n</div>
               </div>
            </div>
         </div>
         <div class=\"grid__status\">line: 2, column: 0</div>
      </div>
   </div>
</div>"#;

    let _out_html = r#"<div class=\"app\">
   <h1>Lines</h1>
   <div>
      <div class=\"grid__wrapper\">
         <div class=\"grid grid__number_wide1\">
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">0</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"623356695095054844\">
               <div class=\"grid__number\">2</div>
               <div class=\"grid__line\">
                  <div>C</div>
                  <div>J</div>
                  <div>K</div>
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">1</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">3</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"9824372840226575955\">
               <div class=\"grid__number\">4</div>
               <div class=\"grid__line\">
                  <div>T</div>
                  <div>h</div>
                  <div>e</div>
                  <div>\n</div>
               </div>
            </div>
         </div>
         <div class=\"grid__status\">line: 2, column: 0</div>
      </div>
   </div>
</div>"#;

    let modified_out = r#"<div class=\"app\">
   <h1>Lines</h1>
   <div>
      <div class=\"grid__wrapper\">
         <div class=\"grid grid__number_wide1\">
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">0</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">1</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"623356695095054844\">
               <div class=\"grid__number\">2</div>
               <div class=\"grid__line\">
                  <div>C</div>
                  <div>J</div>
                  <div>K</div>
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"4638962052468762037\">
               <div class=\"grid__number\">3</div>
               <div class=\"grid__line\">
                  <div>\n</div>
               </div>
            </div>
            <div class=\"grid__number__line\" key=\"9824372840226575955\">
               <div class=\"grid__number\">4</div>
               <div class=\"grid__line\">
                  <div>T</div>
                  <div>h</div>
                  <div>e</div>
                  <div>\n</div>
               </div>
            </div>
         </div>
         <div class=\"grid__status\">line: 2, column: 0</div>
      </div>
   </div>
</div>"#;
    assert_eq!(target_html, modified_out);
}
