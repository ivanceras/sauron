use sauron::*;

#[test]
fn test_eval() {
    let skip_diff = skip_if(true, []);
    let path = TreePath::new([]);
    assert_eq!(skip_diff.eval(path), true);
}

#[test]
fn deep_location() {
    let skip_diff = skip_if(
        true,
        [skip_if(false, []), skip_if(false, [skip_if(true, [])])],
    );
    let path = TreePath::new([1, 0]);
    assert_eq!(skip_diff.eval(path), true);
}

#[test]
fn false_at_non_existent_location() {
    let skip_diff = skip_if(true, []);
    let path = TreePath::new([1, 0]);
    assert_eq!(skip_diff.eval(path), false);
}
