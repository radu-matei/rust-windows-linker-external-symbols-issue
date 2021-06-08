mod external_symbols;

pub fn test_not_using_extern() {}

pub fn abc() {
    let _ = unsafe { external_symbols::foo(1, 2) };
}
