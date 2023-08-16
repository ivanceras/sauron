#![deny(warnings)]
use regex::Regex;

#[test]
fn strip_vdom_id() {
    let replace_data_id = Regex::new(" data-sauron-vdom-id=\"\\d+\"").unwrap();

    let target =   "<main><h1>Minimal example</h1><div class=\"some-class\" id=\"some-id\" data-id=\"1\"><input class=\"client\" type=\"button\" value=\"Click me!\" key=\"1\" data-sauron-vdom-id=\"1\"><div>Clicked: 0</div><input type=\"text\"></div></main>";
    let expected = "<main><h1>Minimal example</h1><div class=\"some-class\" id=\"some-id\" data-id=\"1\"><input class=\"client\" type=\"button\" value=\"Click me!\" key=\"1\"><div>Clicked: 0</div><input type=\"text\"></div></main>";

    println!("target: {}\n", target);

    let target_clean = replace_data_id.replace_all(target, "").into_owned();
    println!("cleaned:{}\n", target_clean);

    assert_eq!(target_clean, expected);
}
