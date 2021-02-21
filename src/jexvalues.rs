#[derive(Debug, Clone, Copy)]
pub enum JexValue {
    NULL,
    INT(i8),
    BOOLEAN(bool),
}

pub fn are_values_equal(x: &JexValue, y: &JexValue) -> bool {
    match (x, y) {
        (JexValue::NULL, JexValue::NULL) => true,
        (JexValue::INT(x), JexValue::INT(y)) => x == y,
        (JexValue::BOOLEAN(x), JexValue::BOOLEAN(y)) => x == y,
        _ => false
    }
}
