#[allow(unused_imports)]
use std::str::FromStr;
use crate::value::*;
use crate::object::*;

#[allow(unused_macros)]
#[macro_export] macro_rules! inline_foreign_function {
    ($input:ident, $body:block) => (
        Value::from_foreign_function(
            |$input| $body
        )
    )
}

#[allow(unused_macros)]
#[macro_export] macro_rules! wrap_foreign_function {
    ($function:expr) => (
        Value::from_foreign_function(
            |input: Value| {$function(input)}
        )
    )
}

#[allow(unused_macros)]
#[macro_export] macro_rules! __obj {
    ($object:ident, $name:meta~$value:expr) => (
        $object.set_attr(stringify!($name).to_string(), $value)
    );

    ($object:ident, $name:meta~$value:expr, $($pass_name:meta~$pass_value:expr),+) => (
        $object.set_attr(stringify!($name).to_string(), $value);
        __obj!($object, $($pass_name~$pass_value),+)
    )
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! obj {
    ($($name:meta~$value:expr),+) => (
        (|| {
            let mut value = Value::new(Type::Instance, NOTHING.to_vec());
            __obj!(
                value, $($name~$value),+
            );
            return value
        })()
    )
}

pub fn empty_obj() -> Value {
    Value::new(Type::Instance, NOTHING.to_vec())
}

pub fn string(s: &str) -> Value {
    Value::from_str(s)
}

pub fn num(n: &str) -> Value {
    Value::from_number(string_to_number(n.to_string()))
}

pub fn none() -> Value {
    Value::from_nothing()
}

pub fn list(arr: &[Value]) -> Value {
    Value::from_vector(arr.to_vec())
}

pub fn fun(arr: &[Value]) -> Value {
    Value::from_function(arr.to_vec())
}

pub fn ins(i: Instruction) -> Value {
    Value::from_instruction(i)
}


// #[allow(unused_macros)]
// #[macro_export] macro_rules! wrap_foreign_function {
//     ($function:expr) => (
//         Value::from_foreign_function(
//             |input: Value| {$function(input)}
//         )
//     )
// }

pub fn foreign_function(fun: fn(Value) -> Value) -> Value {
    Value::from_foreign_function(fun)
}