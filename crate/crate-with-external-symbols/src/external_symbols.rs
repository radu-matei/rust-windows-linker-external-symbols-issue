extern "C" {
    fn foo(x: i32, y: i32) -> i32;
}

pub fn abc() {
    let _ = unsafe { foo(1, 2) };
}
