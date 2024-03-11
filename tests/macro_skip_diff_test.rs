use sauron_macro::*;
use sauron::*;


#[test]
fn skip_if_all_attribute_values_are_static(){
    let skip = skip_diff!{<ul class="some-list" id="some-id"></ul>};
    assert_eq!(skip, sauron::skip_if(true,[]), "skip if all attribute values are static");
}

#[test]
fn dont_skip_if_some_attributes_are_computed(){
    let skip = skip_diff!{<ul class="some-list" id=format!("some-id",100)></ul>};
    assert_eq!(skip, sauron::skip_if(false,[]), "the id is generated");
}

#[test]
fn skip_if_no_attributes(){
    let skip = skip_diff!{<ul></ul>};
    assert_eq!(skip, sauron::skip_if(true,[]), "no attributes");
}

#[test]
fn test_multiple_nodes(){
    let skip = skip_diff!{
        <li></li>
        <li></li>
        <li></li>
    };
    assert_eq!(skip, sauron::skip_if(false,[
            skip_if(true, []),
            skip_if(true, []),
            skip_if(true, []),
    ]), "no attributes");
}

#[test]
fn nested_test_multiple_nodes(){
    let skip = skip_diff!{
        <ul>
            <li></li>
            <li></li>
            <li></li>
        </ul>
    };
    assert_eq!(skip, sauron::skip_if(true,[
            skip_if(true, []),
            skip_if(true, []),
            skip_if(true, []),
    ]), "no attributes");
}
