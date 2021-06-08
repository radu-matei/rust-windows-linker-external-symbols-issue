mod external_symbols;

pub fn test_not_using_extern() {}

pub fn test_using_extern() {
    external_symbols::abc();
}
