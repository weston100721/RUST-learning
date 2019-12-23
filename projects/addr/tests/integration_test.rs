use addr;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!("Hello zhaoliang!", addr::greeting("zhaoliang"));
}