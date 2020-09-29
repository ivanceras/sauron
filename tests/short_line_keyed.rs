#![deny(warnings)]
use sauron::{
    html::attributes::*,
    mt_dom::patch::*,
    *,
};
use sauron_core::Node;

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
    dbg!(&patches);
    assert_eq!(
        patches,
        vec![
            AddAttributes::new(&"div", 1, 1, vec![&key("keyXXX")]).into(),
            ChangeText::new(3, &Text::new("0"), 3, &Text::new("XXX")).into(),
            ChangeText::new(9, &Text::new("1"), 9, &Text::new("2")).into(),
            ChangeText::new(15, &Text::new("2"), 15, &Text::new("3")).into(),
            ChangeText::new(27, &Text::new("3"), 27, &Text::new("4")).into(),
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
    let mut patch = diff(&current_dom, &target_dom);
    patch.sort_by_key(|p| p.priority());
    println!("patch: {:#?}", patch);

    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            ChangeText::new(10, &Text::new("1"), 16, &Text::new("2")).into(),
            ChangeText::new(22, &Text::new("2"), 28, &Text::new("3")).into(),
            ChangeText::new(
                33,
                &Text::new("line: 1, column: 0"),
                39,
                &Text::new("line: 2, column: 0"),
            )
            .into(),
            InsertNode::new(
                Some(&"div"),
                8,
                8,
                &node!(
                <div class="grid__number__line" key="4638962052468762037">
                    <div class="grid__number">"1"</div>
                    <div class="grid__line">
                        <div>"\n"</div>
                    </div>
                </div>)
            )
            .into(),
        ]
    );
    let mut current_dom_clone = current_dom.clone();
    mt_dom::apply_patches(&mut current_dom_clone, &patch);
    assert_eq!(current_dom_clone, target_dom);
}
