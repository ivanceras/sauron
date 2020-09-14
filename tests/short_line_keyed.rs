use log::*;
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    mt_dom::patch::*,
    node,
    *,
};
use sauron_core::{
    body,
    html::div,
    Cmd,
    Component,
    Node,
    Program,
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use wasm_bindgen_test::*;
use web_sys::InputEvent;

#[test]
fn test_unmatched_old_key() {
    let old: Node<()> = node!(
        <div class="grid grid__number_wide1">
            <div class="grid__number__line" key="key000">
                <div class="grid__number">"0"</div>
                <div class="grid__line">
                    <div>"\n"</div>
                </div>
            </div>
            <div class="grid__number__line" key="key111">
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
    );

    let new: Node<()> = node!(
        <div class="grid grid__number_wide1">
            <div class="grid__number__line" key="keyXXX">
                <div class="grid__number">"XXX"</div>
                <div class="grid__line">
                    <div>"\n"</div>
                </div>
            </div>
            <div class="grid__number__line" key="key111">
                <div class="grid__number">"2"</div>
                <div class="grid__line">
                    <div>"\n"</div>
                </div>
            </div>
            <div class="grid__number__line" key="623356695095054844">
                <div class="grid__number">"3"</div>
                <div class="grid__line">
                    <div>"C"</div>
                    <div>"J"</div>
                    <div>"K"</div>
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
    );

    let patches = diff(&old, &new);
    println!("{:#?}", patches);
    assert_eq!(
        patches,
        vec![
            ChangeText::new(9, "1", "2",).into(),
            ChangeText::new(15, "2", "3",).into(),
            ChangeText::new(27, "3", "4",).into(),
            InsertChildren::new(
                &"div",
                0,
                1,
                vec![&div(
                    vec![class("grid__number__line"), key("keyXXX"),],
                    vec![
                        div(vec![class("grid__number")], vec![text("XXX")]),
                        div(
                            vec![class("grid__line")],
                            vec![div(vec![], vec![text("\n")])]
                        ),
                    ]
                )]
            )
            .into(),
            RemoveChildren::new(&"div", 0, vec![0]).into(),
        ]
    );
}

#[test]
fn target_dom() {
    let current_dom: Node<()> = node!(
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
            <div class="grid__status">"line: 1, column: 0"</div>
        </div>
    );

    // Issue: Both 0 and 1 has the same key,
    // Therefore the other 1 is skipped.
    // since the diffin algorithmn threats keys as unique
    let target_dom: Node<()> = node! (
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
            <div class="grid__status">"line: 2, column: 0"</div>
        </div>
    );

    // 1 is lost
    let patch = diff(&current_dom, &target_dom);
    println!("patch: {:#?}", patch);
    let to_insert = node!(
                <div class="grid__number__line" key="4638962052468762037">
                    <div class="grid__number">"1"</div>
                    <div class="grid__line">
                        <div>"\n"</div>
                    </div>
                </div>);
    let expected = vec![
        ChangeText::new(10, "1", "2").into(),
        ChangeText::new(22, "2", "3").into(),
        InsertChildren::new(&"div", 1, 1, vec![&to_insert]).into(),
        ChangeText::new(33, "line: 1, column: 0", "line: 2, column: 0").into(),
    ];
    assert_eq!(patch, expected);
}
