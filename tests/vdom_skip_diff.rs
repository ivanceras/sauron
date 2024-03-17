use sauron::*;
use crate::dom::skip_diff::Marker;

#[test]
fn simple() {
    let skip = skip_if(false, []);
    let path = TreePath::new([]);

    assert_eq!(skip.shall_skip_attributes(&path), false);
}

#[test]
fn simple_skip() {
    let skip = skip_if(true, []);
    let path = TreePath::new([]);

    assert_eq!(skip.shall_skip_attributes(&path), true);
}

#[test]
fn skip_level1() {
    let skip = skip_if(false, [skip_if(true, [])]);
    let path = TreePath::new([0]);

    assert_eq!(skip.shall_skip_attributes(&path), true);
}

#[test]
fn skip_if_not_in_path() {
    let skip = skip_if(false, [skip_if(true, [])]);
    let path = TreePath::new([2, 2]);

    assert_eq!(skip.shall_skip_attributes(&path), true);
}

#[test]
fn dont_collapsed_with_templates() {
    let skip = skip_if(
        true,
        [SkipDiff {
            shall: false,
            marker: Some(Marker::Block),
            children: vec![],
        }],
    );
    let path = TreePath::new([0]);

    assert_eq!(skip.shall_skip_attributes(&path), false);
}

#[test]
fn dont_skip_if_path_has_a_sibling_that_cant_be_skipped() {
    let skip = skip_if(
        true,
        [SkipDiff {
            shall: false,
            marker: Some(Marker::Block),
            children: vec![],
        }],
    );
    let treepath = skip.traverse();
    println!("treepath: {:#?}", treepath);
    let path = TreePath::new([0]);
    let sibling_path = TreePath::new([1]);
    assert_eq!(&treepath[0], &path);

    assert_eq!(skip.shall_skip_attributes(&path), false);
    assert_eq!(skip.has_sibling_template(&sibling_path), true);
}
