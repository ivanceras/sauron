use crate::dom::skip_diff::SkipAttrs;
use sauron::*;

#[test]
fn skip_if_all_attribute_values_are_static() {
    let skip = extract_skip_diff! {<ul class="some-list" id="some-id"></ul>};
    assert_eq!(
        skip,
        sauron::skip_if(true, []),
        "skip if all attribute values are static"
    );
}

#[test]
fn dont_skip_if_some_attributes_are_computed() {
    let skip = extract_skip_diff! {<ul class="some-list" id=format!("some-id",100)></ul>};
    assert_eq!(
        skip,
        sauron::SkipDiff {
            skip_attrs: SkipAttrs::Indices(vec![0]),
            children: vec![]
        },
        "the id is generated"
    );
}

#[test]
fn skip_if_no_attributes() {
    let skip = extract_skip_diff! {<ul></ul>};
    assert_eq!(skip, sauron::skip_if(true, []), "no attributes");
}

#[test]
fn test_multiple_nodes() {
    let skip = extract_skip_diff! {
        <li></li>
        <li></li>
        <li></li>
    };
    assert_eq!(skip, sauron::skip_if(false, []), "no attributes");
}

#[test]
fn nested_test_multiple_nodes() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li></li>
            <li></li>
        </ul>
    };
    assert_eq!(skip, sauron::skip_if(true, []), "no attributes");
}

#[test]
fn test_collapsed() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li></li>
            <li></li>
        </ul>
    };

    assert_eq!(skip, sauron::skip_if(true, []), "can be collapsed");
}

#[test]
fn deep_collapsed() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li>
                <tr></tr>
                <tr></tr>
                <tr></tr>
            </li>
            <li></li>
        </ul>
    };

    assert_eq!(skip, sauron::skip_if(true, []), "can be collapsed");
}

#[test]
fn can_not_collapsed() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li id=format!("id:{}", 1)></li>
            <li></li>
        </ul>
    };

    assert_eq!(
        skip,
        sauron::skip_if(
            true,
            [skip_if(true, []), skip_if(false, []), skip_if(true, [])]
        ),
        "can be collapsed"
    );
}

#[test]
fn partial_collapsed() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li>
                <tr></tr>
                <tr></tr>
                <tr></tr>
            </li>
            <li id=format!("id:{}", 1)></li>
            <li></li>
        </ul>
    };

    assert_eq!(
        skip,
        sauron::skip_if(
            true,
            [
                skip_if(true, []),
                skip_if(true, []),
                skip_if(false, []),
                skip_if(true, [])
            ]
        ),
        "can be collapsed"
    );
}

#[test]
fn collapsed_inside_of_false() {
    let skip = extract_skip_diff! {
        <ul>
            <li></li>
            <li id=format!("id:{}", 1)>
                <tr></tr>
                <tr></tr>
                <tr></tr>
            </li>
            <li></li>
        </ul>
    };

    assert_eq!(
        skip,
        sauron::skip_if(
            true,
            [skip_if(true, []), skip_if(false, []), skip_if(true, [])]
        ),
        "can be collapsed"
    );
}
