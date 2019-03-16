use std::str::FromStr;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul, Div, Rem};
pub use decimal::*;
// use bigdecimal::*;

use crate::error::*;

// type BackendNumber = BigDecimal;
// type BackendNumber = BigRational;
// type BackendNumber = f64;
type BackendNumber = d128;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Number {
    number: BackendNumber,
}

impl Eq for Number {}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.partial_cmp(&other.number).unwrap()
    }
}


fn string_to_backend_number(s: String) -> BackendNumber {
    match BackendNumber::from_str(&s) {
        Ok(k) => k,
        Err(_) => {
            throw_no_stack(&format!("Invalid number: {}", s));
            BackendNumber::from_str("0").unwrap()
        }
    }
    // match s.parse::<BackendNumber>() {
    //     Ok(n) => n,
    //     Err(_) => 0 as BackendNumber
    // }
}

fn backend_number_to_i32(n: &BackendNumber) -> i32 {
    // println!("num to i32");
    // match n.to_usize() {
    //     Some(u) => u,
    //     None => 0 as usize
    // }
    // match n.to_integer().to_i32() {
    //     Some(u) => u,
    //     None => 0
    // }
    return (*n).into()
    // return (*n) as i32
}

fn backend_number_to_usize(n: &BackendNumber) -> usize {
    // match n.to_usize() {
    //     Some(u) => u,
    //     None => 0 as usize
    // }
    // match n.to_integer().to_usize() {
    //     Some(u) => u,
    //     None => 0 as usize
    // }
    return backend_number_to_i32(n) as usize
}

fn backend_number_to_char(n: &BackendNumber) -> char {
    // // match n.to_u8() {
    // //     Some(u) => u as char,
    // //     None => 0 as u8 as char
    // // }
    // match n.to_integer().to_u8() {
    //     Some(u) => u as char,
    //     None => 0 as u8 as char
    // }
    // // n as i32 as u8 as char
    return backend_number_to_i32(n) as u8 as char
}


impl Number {
    fn from_big_rational(number: BackendNumber) -> Self {
        Self {number}
    }

    pub fn from_str(s: &str) -> Self {
        let number = string_to_backend_number(s.to_string());

        Self {number}
    }

    pub fn to_usize(&self) -> usize {
        backend_number_to_usize(&self.number)
    }

    pub fn to_char(&self) -> char {
        backend_number_to_char(&self.number)
    }

    pub fn to_u128(&self) -> u128 {
        backend_number_to_i32(&self.number) as u128
    }

    pub fn unwrap(&self) -> Self {
        // println!("here for debugging");
        self.clone()
    }
}


impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_big_rational(
            self.number + rhs.number
        )
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_big_rational(
            self.number * rhs.number
        )
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_big_rational(
            self.number - rhs.number
        )
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_big_rational(
            self.number / rhs.number
        )
    }
}

impl Rem for Number {
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_big_rational(
            self.number % rhs.number
        )
    }
}


impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // write!(f, "{}", self.number)
        write!(f, "{}", self.number)
    }
}