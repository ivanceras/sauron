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

    enum HigherMsg {
        Msg(Msg),
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

    // cb1 and cb2 has 2 different msg_type_id
    assert_ne!(cb1, cb2);

    // cb2 and cb3 has same msg_type_id, but since it is creating a new closure, then the
    // func_type_id differs
    assert_ne!(cb2, cb3);
    //assert_eq!(cb1, other_cb); //can not compare this one since they have different types

    let map_cb2 = cb2.clone().map_msg(|msg| HigherMsg::Msg(msg));
    let alt_map_cb2 = cb2.map_msg(|msg| HigherMsg::Msg(msg));

    assert_eq!(map_cb2, alt_map_cb2);
}
