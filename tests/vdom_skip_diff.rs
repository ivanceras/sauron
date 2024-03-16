use sauron::*;

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
