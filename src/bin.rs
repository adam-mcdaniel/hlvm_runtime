pub mod stack;
pub mod table;
pub mod object;
pub mod value;
pub mod error;
pub mod literals;

use stack::*;
use object::Instruction::*;
use literals::*;

fn main() {
    StackFrame::from_instructions(
        fun(&[
            string("hey jude"),
            empty_obj(),
            string("a"),
            string("b"),
            string("c"),
            string("d"),
            ins(SetAttr),

            string("self"),
            ins(Store),
            
            string("self"),
            ins(Load),
            ins(Println),


            string("self"),
            ins(Load),

            string("a"),
            string("b"),
            string("c"),
            string("d"),
            ins(GetAttr),
            ins(Println),
        ])
    ).run();
}