use std::fmt::Debug;
use std::process::exit;

pub fn throw<T>(s: &str, stack: Vec<T>) where T: Debug {
    println!("==[ ERROR ]========> {}", s);
    println!("==[ STACK TRACE ]==> {:?}", stack);
    exit(1);
}

pub fn throw_no_stack(s: &str) {
    println!("==[ ERROR ]========> {}", s);
    exit(1);
}