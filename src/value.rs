use std::fmt::{Display};
#[allow(unused_imports)]
use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Not};

use crate::table::*;
use crate::object::*;
use crate::literals::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    value_type: Type,
    function: fn(Self) -> Self,
    contents: Contents,
    list: Vec<Self>,
    attributes: Table<Self>,
}

impl Display for Value {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.print();
        Ok(())
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_number().cmp(&other.as_number())
    }
}


impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        match self.value_type {
            Type::Num => Value::from_number(self.as_number() + rhs.as_number()),
            Type::Str => Value::from_string(self.as_string() + &rhs.as_string()),
            _ => Value::from_problem(Problem::ValueError)
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() - rhs.as_number())
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        // self.println();
        // rhs.println();
        
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() * rhs.as_number())
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() / rhs.as_number())
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() % rhs.as_number())
    }
}

impl Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        match self.value_type {
            Type::Num => {
                if self == num("0") {
                    return num("1");
                } else {
                    return num("0");
                }
            },
            _ => num("0")
        }
    }
}



impl Object for Value {
    fn new(value_type: Type, contents: Contents) -> Self {
        Self {
            value_type: value_type,
            contents: contents,
            function: |object: Self| object,
            list: vec![],
            attributes: Table::new()
        }
    }

    fn get_type(&self) -> Type {self.value_type.clone()}
    fn get_list(&self) -> Vec<Self> {self.list.clone()}
    fn get_contents(&self) -> Contents {self.contents.clone()}
    fn get_attributes(&self) -> Table<Self> {self.attributes.clone()}
    fn get_foreign_function(&self) -> fn(Self) -> Self {self.function.clone()}

    fn set_type(&mut self, value_type: Type) {self.value_type = value_type}
    fn set_list(&mut self, list: Vec<Self>) {self.list = list}
    fn set_contents(&mut self, contents: Contents) {self.contents = contents}
    fn set_attributes(&mut self, attributes: Table<Self>) {self.attributes = attributes}
    fn set_foreign_function(&mut self, function: fn(Self) -> Self) {self.function = function}
}