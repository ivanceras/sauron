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
#[test]
fn test1() {
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
    dbg!(&patch);
    log::trace!("patch: {:#?}", patch);
    //assert_eq!(patch, vec![]);
}
