use log::*;
use sauron::{
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

#[wasm_bindgen_test]
fn multiple_match_on_keyed_elements() {
    console_log::init_with_level(log::Level::Trace);
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

    log::trace!("{:#?}", patches);

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        current_dom.clone(),
        &sauron_core::body(),
    );

    dom_updater.patch_dom(&simple_program, patches);

    let app_node = crate::document()
        .query_selector(".app")
        .expect("must not error")
        .expect("must exist");

    let app_html = app_node.outer_html();

    let mut target_html = String::new();
    target_dom.render_compressed(&mut target_html);

    assert_eq!(target_html, app_html);
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

    let out_html = r#"<div class=\"app\">
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
