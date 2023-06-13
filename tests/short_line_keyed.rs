#![deny(warnings)]
use crate::mt_dom::TreePath;
use sauron::html::*;
use sauron::*;

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
            Patch::replace_node(None, TreePath::new(vec![1, 0, 0,]), &text("2")),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0]),),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&node!(
                <div class="grid__number__line" key="keyxxx">
                    <div class="grid__number">"xxx"</div>
                    <div class="grid__line">
                        <div>"\n"</div>
                    </div>
                </div>)]
            ),
            Patch::replace_node(None, TreePath::new(vec![2, 0, 0,]), &text("3")),
            Patch::replace_node(None, TreePath::new(vec![3, 0, 0,]), &text("4")),
        ]
    );
}
