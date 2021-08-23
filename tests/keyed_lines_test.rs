#![deny(warnings)]
use sauron::{
    html::{attributes::*, *},
    mt_dom::patch::*,
    node, *,
};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_lines() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    let _document = web_sys::window().unwrap().document().unwrap();

    let view0: Node<()> = node!(
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
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="17452480237312370641">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"b"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="16116632904053897079">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"w"</div>
                      <div>"h"</div>
                      <div>"i"</div>
                      <div>"c"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="1175237220632156455">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"t"</div>
                      <div>"o"</div>
                      <div>" "</div>
                      <div>"a"</div>
                      <div>""</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 0, column: 0"</div>
          </div>
       </div>
    </div>
    );

    log::trace!("render: {}", view0.render_to_string());

    let view1: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" key="4638962052468762037">
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="17452480237312370641">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"b"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="16116632904053897079">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"w"</div>
                      <div>"h"</div>
                      <div>"i"</div>
                      <div>"c"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="1175237220632156455">
                   <div class="grid__number">"5"</div>
                   <div class="grid__line">
                      <div>"t"</div>
                      <div>"o"</div>
                      <div>" "</div>
                      <div>"a"</div>
                      <div>""</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 1, column: 0"</div>
          </div>
       </div>
    </div>
        );

    let patch1_diff = diff(&view0, &view1);
    log::trace!("patch1_diff: {:#?}", patch1_diff);

    let inserted = div(
        vec![class("grid__number__line"), key("4638962052468762037")],
        vec![
            div(vec![class("grid__number")], vec![text(0)]),
            div(vec![class("grid__line")], vec![div(vec![], vec![text("")])]),
        ],
    );

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        view0.clone(),
        &sauron_core::body(),
    );

    dom_updater.patch_dom(
        &simple_program,
        vec![
            ChangeText::new(
                &Text::new("0"),
                TreePath::start_at(8, vec![0, 1, 0, 0, 0, 0, 0]),
                &Text::new("1"),
            )
            .into(),
            ChangeText::new(
                &Text::new("1"),
                TreePath::start_at(20, vec![0, 1, 0, 0, 1, 0, 0]),
                &Text::new("2"),
            )
            .into(),
            ChangeText::new(
                &Text::new("2"),
                TreePath::start_at(32, vec![0, 1, 0, 0, 2, 0, 0]),
                &Text::new("3"),
            )
            .into(),
            ChangeText::new(
                &Text::new("3"),
                TreePath::start_at(46, vec![0, 1, 0, 0, 3, 0, 0]),
                &Text::new("4"),
            )
            .into(),
            ChangeText::new(
                &Text::new("4"),
                TreePath::start_at(60, vec![0, 1, 0, 0, 4, 0, 0]),
                &Text::new("5"),
            )
            .into(),
            InsertNode::new(
                Some(&"div"),
                TreePath::start_at(6, vec![0, 1, 0, 0, 0]),
                &inserted,
            )
            .into(),
            ChangeText::new(
                &Text::new("line: 0, column: 0"),
                TreePath::start_at(73, vec![0, 1, 0, 1, 0]),
                &Text::new("line: 1, column: 0"),
            )
            .into(),
        ],
    );
}
