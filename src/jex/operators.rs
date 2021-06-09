use crate::jex::jex_values::get_type::GetType;
use crate::jex::jex_values::to_output_string::ToOutputString;
use crate::jex::jex_values::values::{JexNull, JexObject, JexValue};
use crate::jex::runtime_exceptions::{OperatorNotDefined, TypeException, UnaryOperatorNotDefined};
use crate::machine::exceptions::types::Exception;
use std::rc::Rc;

pub fn plus(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    match (left, right) {
        (JexValue::Int(left), JexValue::Int(right)) => Ok(JexValue::Int(left + right)),
        (JexValue::Object(left), JexValue::Object(right)) => {
            let JexObject::String(left) = &*left;
            let JexObject::String(right) = &*right;
            let result = left.clone() + right;
            Ok(JexValue::Object(Rc::new(JexObject::String(result))))
        }
        (left, right) => Err(Exception::from(OperatorNotDefined::new(
            "plus", &left, &right,
        ))),
    }
}

pub fn minus(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    if let (JexValue::Int(left), JexValue::Int(right)) = (&left, &right) {
        Ok(JexValue::Int(left - right))
    } else {
        Err(Exception::from(OperatorNotDefined::new(
            "minus", &left, &right,
        )))
    }
}

pub fn multiply(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    if let (JexValue::Int(left), JexValue::Int(right)) = (&left, &right) {
        Ok(JexValue::Int(left * right))
    } else {
        Err(Exception::from(OperatorNotDefined::new(
            "multiply", &left, &right,
        )))
    }
}

pub fn divide(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    if let (JexValue::Int(left), JexValue::Int(right)) = (&left, &right) {
        Ok(JexValue::Int(left / right))
    } else {
        Err(Exception::from(OperatorNotDefined::new(
            "divide", &left, &right,
        )))
    }
}

pub fn negate(value: JexValue) -> Result<JexValue, Exception> {
    if let JexValue::Int(int) = value {
        Ok(JexValue::Int(-int))
    } else {
        Err(Exception::from(UnaryOperatorNotDefined::new(
            "negate", &value,
        )))
    }
}

pub fn not(value: JexValue) -> Result<JexValue, Exception> {
    if let JexValue::Bool(bool) = value {
        Ok(JexValue::Bool(!bool))
    } else {
        Err(Exception::from(UnaryOperatorNotDefined::new("not", &value)))
    }
}

pub fn to_string(value: JexValue) -> Result<JexValue, Exception> {
    Ok(JexValue::Object(Rc::new(JexObject::String(JexValue::to_output_string(&value)))))
}

pub fn equal(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    Ok(JexValue::Bool(left == right))
}

pub fn greater(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    if let (JexValue::Int(left), JexValue::Int(right)) = (&left, &right) {
        Ok(JexValue::Bool(left > right))
    } else {
        Err(Exception::from(OperatorNotDefined::new(
            "greater", &left, &right,
        )))
    }
}

pub fn less(left: JexValue, right: JexValue) -> Result<JexValue, Exception> {
    if let (JexValue::Int(left), JexValue::Int(right)) = (&left, &right) {
        Ok(JexValue::Bool(left < right))
    } else {
        Err(Exception::from(OperatorNotDefined::new(
            "less", &left, &right,
        )))
    }
}

pub fn print(value: JexValue) -> Result<JexValue, Exception> {
    println!("{}", value.to_output_string());
    Ok(JexValue::Null(JexNull))
}

#[cfg(test)]
mod tests {
    use crate::jex::jex_values::values::{JexValue, JexObject};
    use crate::jex::operators::{divide, equal, greater, less, minus, multiply, negate, not, plus, to_string};
    use std::rc::Rc;

    #[test]
    fn plus_should_add_two_ints() {
        assert_eq!(
            JexValue::Int(100),
            plus(JexValue::Int(80), JexValue::Int(20)).unwrap()
        );
    }

    #[test]
    fn plus_should_add_two_strings() {
        let left = JexValue::from_string("A".to_string());
        let right = JexValue::from_string("BC".to_string());
        let expected = JexValue::from_string("ABC".to_string());

        assert_eq!(expected, plus(left, right).unwrap());
    }

    #[test]
    fn minus_should_subtract_ints() {
        assert_eq!(
            JexValue::Int(-50),
            minus(JexValue::Int(20), JexValue::Int(70)).unwrap()
        );
    }

    #[test]
    fn multiply_should_multiply_ints() {
        assert_eq!(
            JexValue::Int(-50),
            multiply(JexValue::Int(2), JexValue::Int(-25)).unwrap()
        );
    }

    #[test]
    fn divide_should_divide_ints() {
        assert_eq!(
            JexValue::Int(-33),
            divide(JexValue::Int(100), JexValue::Int(-3)).unwrap()
        );
    }

    #[test]
    fn negate_should_negate_int() {
        assert_eq!(JexValue::Int(-33), negate(JexValue::Int(33)).unwrap());
    }

    #[test]
    fn not_should_negate_bool() {
        assert_eq!(JexValue::Bool(true), not(JexValue::Bool(false)).unwrap());
    }

    #[test]
    fn to_string_should_convert_bool_to_string() {
        assert_eq!(JexValue::Object(Rc::new(JexObject::String(String::from("false")))), to_string(JexValue::Bool(false)).unwrap());
    }

    #[test]
    fn to_string_should_convert_int_to_string() {
        assert_eq!(JexValue::Object(Rc::new(JexObject::String(String::from("69")))), to_string(JexValue::Int(69)).unwrap());
    }

    #[test]
    fn two_same_ints_should_be_equal() {
        assert_eq!(
            JexValue::Bool(true),
            equal(JexValue::Int(100), JexValue::Int(100)).unwrap()
        );
    }

    #[test]
    fn two_different_ints_should_be_not_equal() {
        assert_eq!(
            JexValue::Bool(false),
            equal(JexValue::Int(10), JexValue::Int(100)).unwrap()
        );
    }

    #[test]
    fn two_same_strings_should_be_equal() {
        assert_eq!(
            JexValue::Bool(true),
            equal(
                JexValue::from_string("aab".to_string()),
                JexValue::from_string("aab".to_string())
            )
            .unwrap()
        );
    }

    #[test]
    fn test_10_should_be_greater_than_minus_100() {
        assert_eq!(
            JexValue::Bool(true),
            greater(JexValue::Int(10), JexValue::Int(-100)).unwrap()
        );
    }

    #[test]
    fn test_10_should_be_less_than_50() {
        assert_eq!(
            JexValue::Bool(true),
            less(JexValue::Int(10), JexValue::Int(50)).unwrap()
        );
    }
}
