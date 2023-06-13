#![deny(warnings)]
use sauron::Listener;
use sauron_core::*;

#[test]
fn test_type_ids() {
    enum Msg {
        Click(usize),
        Hover(i32),
    }

    enum ParentMsg {
        Other,
    }

    let cb1 = Listener::from(|_e| Msg::Click(1));
    let cb2 = Listener::from(|_e| Msg::Hover(2));
    let cb3 = Listener::from(|_e| Msg::Hover(3));
    let cb4 = Listener::from(|_e| Msg::Hover(3));

    let f1 = |_e| Msg::Click(1);
    let fcb1 = Listener::from(f1);
    let fcb2 = Listener::from(f1);

    dbg!(&fcb1);
    dbg!(&fcb2);
    assert_eq!(fcb1, fcb2);

    dbg!(&cb1);
    dbg!(&cb2);
    dbg!(&cb3);
    dbg!(&cb4);

    let other_cb = Listener::from(|_e| ParentMsg::Other);
    dbg!(&other_cb);

    //assert_eq!(cb1, cb2);
    //assert_eq!(cb2, cb3);

    //assert_eq!(cb1, other_cb); //can not compare this one since they have different types
}
