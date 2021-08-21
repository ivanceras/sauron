use sauron::Listener;
use sauron_core::prelude::*;

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

    let _other_cb = Listener::from(|_e| ParentMsg::Other);

    assert_eq!(cb2, cb3);
    assert_eq!(cb1, cb2);
}
