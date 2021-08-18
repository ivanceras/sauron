use sauron_core::prelude::*;

#[test]
fn test_type_ids() {
    enum Msg {
        Click(usize),
        Hover(i32),
    }

    println!("click1:");
    let old: Node<Msg> = div(vec![on_click(|_e| Msg::Click(1))], vec![]);
    println!("hover2:");
    let new: Node<Msg> = div(vec![on_mouseover(|_e| Msg::Hover(2))], vec![]);
    println!("hover3:");
    let new2: Node<Msg> = div(vec![on_mouseover(|_e| Msg::Hover(3))], vec![]);
    panic!();
}
