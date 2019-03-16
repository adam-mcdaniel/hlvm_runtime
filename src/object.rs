
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div, Rem, Not};

use crate::number::Number;
use crate::error::*;
use crate::table::Table;

pub type Contents = Vec<Number>;
pub const NOTHING : &[Number] = &[];


fn float64_to_number(num: f64) -> Number {
    Number::from_str(&num.to_string())
}


fn from_string(string: String) -> Contents {
    let mut result = vec![];
    for ch in string.chars() {
        result.push(
            Number::from_str(&(ch as i32).to_string())
        );
    }
    return result;
}


fn from_number(number: Number) -> Contents {
    return vec![number];
}


pub fn string_to_number(n: String) -> Number {
    // println!("String to num");
    // match Number::from_str(&n) {
    //     Ok(s) => s,
    //     Err(_) => return Number::from_str("0").unwrap()
    // }
    // match n.parse::<f64>() {
    //     Ok(n) => n,
    //     Err(_) => 0 as f64
    // }
    Number::from_str(&n)
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Print,
    Println,
    While,
    If,
    Append,
    Pop,
    Index,
    Equal,
    Greater,
    Less,
    Not,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    Call,
    Load,
    Store,
    GetAttr,
    SetAttr,
    Execute,
    Pass
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Problem {
    IncompatibleTypes,
    ValueError,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Str,
    Num,
    List,
    Function,
    Instance,
    Problem(Problem),
    Nothing,
    Command(Instruction)
}


pub trait Object: Sized + Clone + Debug + Display + Add + Sub + Mul + Div + Rem + Ord + Not {
    // initializers
    fn new(value_type: Type, contents: Contents) -> Self;

    fn empty_instance() -> Self {
        return Self::new(Type::Instance, NOTHING.to_vec());
    }

    fn from_string(string: String) -> Self {
        return Self::new(Type::Str, from_string(string));
    }

    fn from_str(string: &str) -> Self {
        return Self::new(Type::Str, from_string(string.to_string()));
    }

    fn from_f64(decimal: f64) -> Self {
        return Self::new(Type::Num, from_number(float64_to_number(decimal)));
    }

    fn from_number(n: Number) -> Self {
        return Self::new(Type::Num, from_number(n));
    }

    fn from_instruction(instruction: Instruction) -> Self {
        return Self::new(Type::Command(instruction), NOTHING.to_vec());
    }

    fn from_problem(problem: Problem) -> Self {
        return Self::new(Type::Problem(problem), NOTHING.to_vec());
    }

    fn from_vector(vector: Vec<Self>) -> Self {
        let mut instance = Self::new(Type::List, NOTHING.to_vec());
        instance.set_list(vector);
        return instance;
    }

    fn from_nothing() -> Self {
        Self::new(
            Type::Nothing,
            NOTHING.to_vec()
            )
    }

    fn from_function(vector: Vec<Self>) -> Self {
        let mut instance = Self::new(Type::Function, NOTHING.to_vec());
        instance.set_list(vector);
        return instance;
    }

    fn from_foreign_function(function: fn(Self) -> Self) -> Self {
        let mut instance = Self::new(
            Type::Function,
            NOTHING.to_vec()
            );
        instance.set_foreign_function(function);
        return instance;
    }

    // helper functions
    fn get_type(&self) -> Type;
    fn get_list(&self) -> Vec<Self>;
    fn get_contents(&self) -> Contents;
    fn get_attributes(&self) -> Table<Self>;
    fn get_foreign_function(&self) -> fn(Self) -> Self;

    fn set_type(&mut self, object_type: Type);
    fn set_list(&mut self, list: Vec<Self>);
    fn set_contents(&mut self, contents: Contents);
    fn set_attributes(&mut self, attributes: Table<Self>);
    fn set_foreign_function(&mut self, function: fn(Self) -> Self);

    // getters
    fn get_attr(&self, name: String) -> Self {
        let table = self.get_attributes();
        let raw_attr = table.get(name);
        let attr = match raw_attr {
            Some(s) => s,
            None => Self::new(Type::Nothing, NOTHING.to_vec())
        };
        return attr;
    }

    fn as_number(&self) -> Number {
        if self.get_contents().len() > 0 {
            self.get_contents()[0].clone()
        } else {
            // Number::from_str("0").unwrap()
            string_to_number("0".to_string())
        }
    }

    fn as_usize(&self) -> usize {
        if self.get_contents().len() > 0 {
            // match self.get_contents()[0].to_i32() {
            //     Some(i) => i as usize,
            //     None => 0 as usize
            // }
            self.get_contents()[0].to_usize()
        } else {
            0 as usize
        }
    }

    fn as_string(&self) -> String {
        let mut result = "".to_string();

        for ch in self.get_contents() {
            // let character = match ch.to_i32() {
            //     Some(i) => i as u8 as char,
            //     None => ' '
            // };
            let character = ch.to_char();
            result += &character.to_string();
        }
        return result.to_string();
    }
    
    fn as_list(&self) -> Vec<Self> {
        self.get_list()
    }

    fn as_instruction(&self) -> Instruction {
        match self.get_type() {
            Type::Command(i) => i,
            _ => Instruction::Pass
        }
    }
    
    fn as_instance(&self) -> Table<Self> {
        return self.get_attributes();
    }
    
    
    fn as_foreign_function(&self) -> fn(Self) -> Self {
        return self.get_foreign_function();
    }
    
    // setters
    fn set_attr(&mut self, name: String, object: Self) {
        let mut table = self.get_attributes();
        table.set(name, object);
        self.set_attributes(table);
    }

    fn get_attr_recursive(&mut self, names: Vec<String>) -> Self {
        if names.len() < 1 {
            throw_no_stack("Could not set attribute of object without the attribute name");
        }

        let name = &names[0];
        let table = self.get_attributes();
        if names.len() == 1 {

            match table.get(name.to_string()) {
                Some(o) => o,
                None => Self::from_nothing()
            }

        } else {

            match table.get(name.to_string()) {
                Some(o) => o,
                None => Self::empty_instance()
            }.get_attr_recursive(names[1..].to_vec())
        }
    }

    fn set_attr_recursive(&mut self, names: Vec<String>, object: Self) -> Self {
        if names.len() < 1 {
            throw_no_stack("Could not set attribute of object without the attribute name");
        }

        let name = &names[0];
        let mut table = self.get_attributes();

        if names.len() == 1 {

            table.set(name.to_string(), object);
            self.set_attributes(table);

        } else {

            table.set(
                name.to_string(),
                match table.get(name.to_string()) {
                    Some(o) => o,
                    None => Self::empty_instance()
                }
                    .set_attr_recursive(names[1..].to_vec(), object));
            self.set_attributes(table);

        }
        self.clone()
    }


    fn index(&mut self, index: Self) -> Self {
        let my_type = self.get_type();
        match my_type {
            Type::Str => match self.as_string().chars().nth(index.as_usize()) {
                Some(c) => {
                    let mut string = String::new();
                    string.push(c);
                    Self::from_string(string)
                },
                None => {
                    Self::from_problem(
                        Problem::OutOfRange
                    )
                }
            },
            Type::List => self.as_list()[index.as_usize()].clone(),
            Type::Function => self.as_list()[index.as_usize()].clone(),
            _ => Self::from_problem(Problem::ValueError)
        }
    }

    fn list_push(&mut self, object: Self) {
        self.set_type(Type::List);

        let mut list = self.get_list();
        list.push(object);
        self.set_list(list);
    }

    fn list_pop(&mut self) -> Self {
        match self.get_list().pop() {
            Some(e) => e,
            None => Self::new(Type::Nothing, NOTHING.to_vec())
        }
    }

    fn call_foreign_function(&mut self, parameter: Self) -> Self {
        self.get_foreign_function()(parameter)
    }

    fn format(&self) -> String {
        let object_type = self.get_type();
        match object_type {
            Type::Str => format!("{}", self.as_string()),
            Type::Num => format!("{}", self.as_number()),
            Type::List => {
                if self.as_list().len() == 0 {
                    return "[]".to_string();
                }
                let mut result = "[".to_string();
                for item in self.as_list() {
                    result += &item.format();
                    result += ", ";
                }
                result.pop();
                result.pop();
                result + "]"
                },
            Type::Instance => {

                if self.get_attributes().keys().len() < 1 {
                    "<>".to_string()
                } else {
                    let mut result = "<".to_string();
                    for key in self.get_attributes().keys() {
                        result += &key;
                        result += ":";
                        result += &self.get_attr(key).format();
                        result += ", ";
                    }
                    result.pop();
                    result.pop();
                    result + ">"
                }

            },
            Type::Function => format!("Function"),
            Type::Nothing => format!("None"),
            Type::Problem(p) => format!("{:?}", p),
            Type::Command(c) => format!("{:?}", c),
        }
    }

    fn print(&self) {
        print!("{}", self.format());
    }

    fn println(&self) {
        println!("{}", self.format());
    }
}
