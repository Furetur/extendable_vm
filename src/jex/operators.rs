use crate::jex::jex_values::to_output_string::ToOutputString;
use crate::jex::jex_values::values::{JexNull, JexObject, JexValue};
use crate::jex::runtime_exceptions::{OperatorNotDefined, UnaryOperatorNotDefined};
use crate::jex::types::JexMachine;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use scanrs::scanln;
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
    Ok(JexValue::from_string(value.to_output_string()))
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

// TODO: should be a nullary operator
pub fn read_line(machine: &mut JexMachine, mut _args: InstructionPointer) -> Result<(), Exception> {
    let line = scanln();
    let value = JexValue::from_string(line.to_string());
    machine.push_operand(value);
    Ok(())
}

pub fn parse_int(value: JexValue) -> Result<JexValue, Exception> {
    if let Some(str) = value.as_string() {
        Ok(str
            .parse::<i32>()
            .map_or_else(|_e| JexValue::null(), |int| JexValue::Int(int)))
    } else {
        Err(Exception::from(UnaryOperatorNotDefined::new(
            "PARSE_INT",
            &value,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::jex::jex_values::values::{JexObject, JexValue};
    use crate::jex::operators::{
        divide, equal, greater, less, minus, multiply, negate, not, parse_int, plus, to_string,
    };

    // PLUS

    #[test]
    fn plus_should_add_two_ints() {
        assert_eq!(
            JexValue::Int(100),
            plus(JexValue::Int(80), JexValue::Int(20)).unwrap()
        );
    }

    #[test]
    fn plus_should_concat_two_strings() {
        let left = JexValue::from_string("A".to_string());
        let right = JexValue::from_string("BC".to_string());
        let expected = JexValue::from_string("ABC".to_string());

        assert_eq!(expected, plus(left, right).unwrap());
    }

    #[test]
    fn plus_should_not_add_two_bools() {
        assert!(plus(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    #[test]
    fn plus_should_not_add_int_and_null() {
        assert!(plus(JexValue::Int(1), JexValue::null()).is_err());
    }

    // MINUS

    #[test]
    fn minus_should_subtract_ints() {
        assert_eq!(
            JexValue::Int(-50),
            minus(JexValue::Int(20), JexValue::Int(70)).unwrap()
        );
    }

    #[test]
    fn minus_should_not_subtract_two_bools() {
        assert!(minus(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    #[test]
    fn minus_should_not_subtract_int_and_null() {
        assert!(minus(JexValue::Int(1), JexValue::null()).is_err());
    }

    // MULTIPLY

    #[test]
    fn multiply_should_multiply_ints() {
        assert_eq!(
            JexValue::Int(-50),
            multiply(JexValue::Int(2), JexValue::Int(-25)).unwrap()
        );
    }

    #[test]
    fn multiply_should_not_mul_two_bools() {
        assert!(multiply(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    #[test]
    fn multiply_should_not_mul_int_and_null() {
        assert!(multiply(JexValue::Int(1), JexValue::null()).is_err());
    }

    // DIVIDE

    #[test]
    fn divide_should_divide_ints() {
        assert_eq!(
            JexValue::Int(-20),
            divide(JexValue::Int(100), JexValue::Int(-5)).unwrap()
        );
    }

    #[test]
    fn divide_should_divide_ints_with_floor() {
        assert_eq!(
            JexValue::Int(-33),
            divide(JexValue::Int(100), JexValue::Int(-3)).unwrap()
        );
    }

    #[test]
    fn divide_should_not_divide_two_bools() {
        assert!(divide(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    #[test]
    fn divide_should_not_div_int_and_null() {
        assert!(divide(JexValue::Int(1), JexValue::null()).is_err());
    }

    // NEGATE

    #[test]
    fn negated_33_should_be_minus_33() {
        assert_eq!(JexValue::Int(-33), negate(JexValue::Int(33)).unwrap());
    }

    #[test]
    fn negate_should_not_work_for_bool() {
        assert!(negate(JexValue::Bool(true)).is_err());
    }

    #[test]
    fn negate_should_not_work_for_null() {
        assert!(negate(JexValue::null()).is_err());
    }

    // TO STRING

    #[test]
    fn to_string_should_convert_bool_to_string() {
        assert_eq!(
            JexValue::from_string(String::from("false")),
            to_string(JexValue::Bool(false)).unwrap()
        );
    }

    #[test]
    fn to_string_should_convert_int_to_string() {
        assert_eq!(
            JexValue::from_string(String::from("69")),
            to_string(JexValue::Int(69)).unwrap()
        );
    }

    // NOT

    #[test]
    fn not_false_should_be_true_and_not_true_should_be_false() {
        assert_eq!(JexValue::Bool(true), not(JexValue::Bool(false)).unwrap());
        assert_eq!(JexValue::Bool(false), not(JexValue::Bool(true)).unwrap());
    }

    #[test]
    fn not_should_not_work_for_ints() {
        assert!(not(JexValue::Int(1)).is_err());
    }

    #[test]
    fn not_should_not_work_for_null() {
        assert!(not(JexValue::null()).is_err());
    }

    // EQUAL

    #[test]
    fn test_100_is_equal_to_100() {
        assert_eq!(
            JexValue::Bool(true),
            equal(JexValue::Int(100), JexValue::Int(100)).unwrap()
        );
    }

    #[test]
    fn test_10_is_not_equal_to_100() {
        assert_eq!(
            JexValue::Bool(false),
            equal(JexValue::Int(10), JexValue::Int(100)).unwrap()
        );
    }

    #[test]
    fn true_and_true_should_be_equal() {
        assert_eq!(
            JexValue::Bool(true),
            equal(JexValue::Bool(true), JexValue::Bool(true)).unwrap()
        );
    }

    #[test]
    fn false_and_false_should_be_equal() {
        assert_eq!(
            JexValue::Bool(true),
            equal(JexValue::Bool(false), JexValue::Bool(false)).unwrap()
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
    fn two_different_strings_should_not_be_equal() {
        assert_eq!(
            JexValue::Bool(false),
            equal(
                JexValue::from_string("aab".to_string()),
                JexValue::from_string("aabcd".to_string())
            )
            .unwrap()
        );
    }

    #[test]
    fn test_null_is_equal_to_null() {
        assert_eq!(
            JexValue::Bool(true),
            equal(JexValue::null(), JexValue::null()).unwrap()
        );
    }

    #[test]
    fn test_null_is_not_equal_to_100() {
        assert_eq!(
            JexValue::Bool(false),
            equal(JexValue::null(), JexValue::Int(100)).unwrap()
        );
    }

    #[test]
    fn test_null_is_not_equal_to_true() {
        assert_eq!(
            JexValue::Bool(false),
            equal(JexValue::null(), JexValue::Bool(true)).unwrap()
        );
    }

    // GREATER

    #[test]
    fn test_10_should_be_greater_than_minus_100() {
        assert_eq!(
            JexValue::Bool(true),
            greater(JexValue::Int(10), JexValue::Int(-100)).unwrap()
        );
    }

    #[test]
    fn test_0_should_not_be_greater_than_10() {
        assert_eq!(
            JexValue::Bool(false),
            greater(JexValue::Int(0), JexValue::Int(10)).unwrap()
        );
    }

    #[test]
    fn test_3_should_not_be_greater_than_3() {
        assert_eq!(
            JexValue::Bool(false),
            greater(JexValue::Int(3), JexValue::Int(3)).unwrap()
        );
    }

    #[test]
    fn greater_should_not_compare_bools() {
        assert!(greater(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    // LESS

    #[test]
    fn test_10_should_be_less_than_50() {
        assert_eq!(
            JexValue::Bool(true),
            less(JexValue::Int(10), JexValue::Int(50)).unwrap()
        );
    }

    #[test]
    fn test_100_should_not_be_less_than_10() {
        assert_eq!(
            JexValue::Bool(false),
            less(JexValue::Int(100), JexValue::Int(10)).unwrap()
        );
    }

    #[test]
    fn test_3_should_not_be_less_than_3() {
        assert_eq!(
            JexValue::Bool(false),
            less(JexValue::Int(3), JexValue::Int(3)).unwrap()
        );
    }

    #[test]
    fn less_should_not_compare_bools() {
        assert!(less(JexValue::Bool(true), JexValue::Bool(false)).is_err());
    }

    // PARSE INT

    #[test]
    fn parse_int_should_parse_1() {
        let string = JexValue::from_string("1".to_string());
        let actual = parse_int(string).unwrap();
        assert_eq!(actual, JexValue::Int(1));
    }

    #[test]
    fn parse_int_should_parse_negative_100() {
        let string = JexValue::from_string("-100".to_string());
        let actual = parse_int(string).unwrap();
        assert_eq!(actual, JexValue::Int(-100));
    }

    #[test]
    fn parse_int_should_parse_negative_99999() {
        let string = JexValue::from_string("-99999".to_string());
        let actual = parse_int(string).unwrap();
        assert_eq!(actual, JexValue::Int(-99999));
    }

    #[test]
    fn parse_int_should_return_null_if_given_abc() {
        let string = JexValue::from_string("abc".to_string());
        let actual = parse_int(string).unwrap();
        assert_eq!(actual, JexValue::null());
    }

    #[test]
    fn parse_int_should_return_null_if_given_100a() {
        let string = JexValue::from_string("100a".to_string());
        let actual = parse_int(string).unwrap();
        assert_eq!(actual, JexValue::null());
    }
}
