#![deny(warnings)]
use sauron::EventCallback;

#[test]
fn test_type_ids() {
    enum Msg {
        Click(usize),
        Hover(i32),
    }

    enum ParentMsg {
        Other,
    }

    let cb1 = EventCallback::from(|_e| Msg::Click(1));
    let cb2 = EventCallback::from(|_e| Msg::Hover(2));
    let cb3 = EventCallback::from(|_e| Msg::Hover(3));
    let cb4 = EventCallback::from(|_e| Msg::Hover(3));

    let f1 = |_e| Msg::Click(1);
    let fcb1 = EventCallback::from(f1);
    let fcb2 = EventCallback::from(f1);

    dbg!(&fcb1);
    dbg!(&fcb2);
    assert_eq!(fcb1, fcb2);

    dbg!(&cb1);
    dbg!(&cb2);
    dbg!(&cb3);
    dbg!(&cb4);

    let other_cb = EventCallback::from(|_e| ParentMsg::Other);
    dbg!(&other_cb);

    //assert_eq!(cb1, cb2);
    //assert_eq!(cb2, cb3);

    //assert_eq!(cb1, other_cb); //can not compare this one since they have different types
}
