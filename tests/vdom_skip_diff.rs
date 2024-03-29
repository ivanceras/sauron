use sauron::dom::skip_diff::SkipAttrs;
use sauron::*;

#[test]
fn simple() {
    let skip = skip_if(false, []);
    let path = TreePath::new([]);

    assert_eq!(skip.in_path(&path).unwrap().shall_skip_attributes(), false);
}

#[test]
fn simple_skip() {
    let skip = skip_if(true, []);
    let path = TreePath::new([]);

    assert_eq!(skip.in_path(&path).unwrap().shall_skip_attributes(), true);
}

#[test]
fn skip_level1() {
    let skip = skip_if(false, [skip_if(true, [])]);
    let path = TreePath::new([0]);

    assert_eq!(skip.in_path(&path).unwrap().shall_skip_attributes(), true);
}

#[test]
fn dont_collapsed_with_templates() {
    let skip = skip_if(
        true,
        [SkipDiff {
            skip_attrs: SkipAttrs::none(),
            children: vec![],
        }],
    );
    let path = TreePath::new([0]);

    assert_eq!(skip.in_path(&path).unwrap().shall_skip_attributes(), false);
}

#[test]
fn dont_skip_if_path_has_a_sibling_that_cant_be_skipped() {
    let skip = skip_if(
        true,
        [SkipDiff {
            skip_attrs: SkipAttrs::none(),
            children: vec![],
        }],
    );
    let path = TreePath::new([0]);

    assert_eq!(skip.in_path(&path).unwrap().shall_skip_attributes(), false);
}
