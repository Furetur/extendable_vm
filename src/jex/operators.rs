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
