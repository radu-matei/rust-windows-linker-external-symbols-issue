pub fn test_not_using_extern() {}

pub fn abc() {
    let _ = unsafe { foo(1, 2) };
}

extern "C" {
    fn foo(x: i32, y: i32) -> i32;
}
