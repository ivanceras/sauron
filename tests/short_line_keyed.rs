#![deny(warnings)]
use sauron::{mt_dom::patch::*, *};
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
            <div class="grid__number__line" key="keyxxx">
                <div class="grid__number">"xxx"</div>
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
            ReplaceNode::new(
                Some(&"div"),
                PatchPath::new(
                    TreePath::start_at(1, vec![0, 0]),
                    TreePath::start_at(1, vec![0, 0])
                ),
                &node!(
                <div class="grid__number__line" key="keyxxx">
                    <div class="grid__number">"xxx"</div>
                    <div class="grid__line">
                        <div>"\n"</div>
                    </div>
                </div>)
            )
            .into(),
            ChangeText::new(
                &Text::new("1"),
                PatchPath::new(
                    TreePath::start_at(9, vec![0, 1, 0, 0,]),
                    TreePath::start_at(9, vec![0, 1, 0, 0,])
                ),
                &Text::new("2")
            )
            .into(),
            ChangeText::new(
                &Text::new("2"),
                PatchPath::new(
                    TreePath::start_at(15, vec![0, 2, 0, 0,]),
                    TreePath::start_at(15, vec![0, 2, 0, 0,])
                ),
                &Text::new("3")
            )
            .into(),
            ChangeText::new(
                &Text::new("3"),
                PatchPath::new(
                    TreePath::start_at(27, vec![0, 3, 0, 0,]),
                    TreePath::start_at(27, vec![0, 3, 0, 0,])
                ),
                &Text::new("4")
            )
            .into(),
        ]
    );
}
