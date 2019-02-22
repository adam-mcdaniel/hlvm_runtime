use crate::object::*;
use crate::value::*;
use crate::table::Table;

#[derive(Debug, Clone)]
pub struct Pair<A, B> {
    first: A,
    second: B
}

#[derive(Debug, Clone)]
pub struct Scope {
    table: Table<Pair<Value, Scope>>, // the variables in scope
    outer_scope: Option<Box<Scope>>   // the pointer to the parent scope
}

impl Scope {
    // create a new scope from a parent scope
    pub fn new(outer_scope: Option<Box<Scope>>) -> Self {
        return Self {
            table: Table::new(),
            outer_scope
        }
    }

    // define the value of the variable in this scope
    fn define(&mut self, name: String, object: Pair<Value, Scope>) {
        self.table.set(name, object)
    }

    // get the value of the variable in this scope
    fn get(&mut self, name: String) -> Pair<Value, Scope> {
        match self.table.get(name.clone()) {
            Some(v) => return v,
            None => {
                // the value isnt in this scope
                // try to use the parent scope if possible, 
                // otherwise just return an empty value and scope
                (*match self.outer_scope.clone() {
                    Some(v) => v,
                    None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
                }).get(name)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    scope: Scope,                        // scope used to store variables
    contents: Vec<Pair<Value, Scope>>,   // stores the contents of the stack
    instructions: Value,                 // the instructions to run (a function or a list)
    number_of_args_taken: i32,           // used to count how many arguments a stack used
    outer_stack: Option<Box<StackFrame>> // the parent stack
}

impl StackFrame {
    // create stackframe from a parent stackframe / scope
    pub fn new(outer_stack: Option<Box<StackFrame>>, scope: Scope, instructions: Value) -> Self {
        return Self {
            scope,
            contents: vec![],
            instructions,
            number_of_args_taken: 0,
            outer_stack
        }
    }

    // create a new stackframe from a function
    pub fn from_instructions(instructions: Value) -> Self {
        Self {
            scope: Scope::new(None),
            contents: vec![],
            instructions,
            number_of_args_taken: 0,
            outer_stack: None
        }
    }

    // run a stackframe
    pub fn run(&mut self) {
        for instruction in self.instructions.as_list() {
            match instruction.as_instruction() {
                // print the topmost object without a carriage return
                Instruction::Print => self.pop_value().print(),
                // print the topmost object with a carriage return
                Instruction::Println => self.pop_value().println(),

                // call the topmost object on the stack as
                // a function (as if it were in this scope)
                // until it returns 0
                Instruction::While => {
                    let f = self.pop();
                    self.while_function(f);
                },

                // add the topmost objects
                Instruction::Append => {
                    let mut list = self.pop_value();
                    let value = self.pop_value();
                    list.list_push(value);
                    self.push_value(list);
                },

                // multiply the topmost objects
                Instruction::Pop => {
                    let mut list = self.pop_value();
                    let value = list.list_pop();
                    self.push_value(value);
                },
                
                // subtract the topmost objects
                Instruction::Index => {
                    let mut list = self.pop_value();
                    let index = self.pop_value();
                    self.push_value(list.index(index));
                },
                

                // add the topmost objects
                Instruction::Add => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a + b);
                },

                // multiply the topmost objects
                Instruction::Mul => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a * b);
                },
                
                // subtract the topmost objects
                Instruction::Sub => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a - b);
                },
                
                // divide the topmost objects
                Instruction::Div => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a / b);
                },

                // apply the % operator to the topmost objects
                Instruction::Mod => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a % b);
                },
                
                // call the topmost object on the stack as a function
                Instruction::Call => {
                    let f = self.pop();
                    self.call(f);
                },

                // load a variable with a given name
                Instruction::Load => {
                    let name = self.pop_value().as_string();
                    let value = self.load(name);
                    self.push(value);
                },

                // store takes a name and a value
                // and stores the value under that name
                // as a variable that can be loaded
                Instruction::Store => {
                    let name = self.pop_value().as_string();
                    let value = self.pop();
                    self.store(name, value);
                },

                // getattr retreives an attribute of an object
                Instruction::GetAttr => {
                    let name = self.pop_value().as_string();
                    let object = self.pop_value();
                    self.push_value(object.get_attr(name));
                },

                // setattr modifies an attribute of an object
                Instruction::SetAttr => {
                    let name = self.pop_value().as_string();
                    let data = self.pop_value();
                    let mut object = self.pop_value();
                    object.set_attr(name, data);
                    self.push_value(object);
                },

                // execute takes the topmost object on the stack
                // and executes it as a foreign function
                // a foreign function takes a Value and returns
                // a Value
                Instruction::Execute => {
                    let mut foreign_function = self.pop_value();
                    let argument = self.pop_value();
                    self.push_value(
                        foreign_function.call_foreign_function(argument)
                        );
                },

                // pass does nothing
                Instruction::Pass => self.push_value(instruction)
            }
        }
    }

    // is the stack empty?
    fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    // does this stack frame have a valid outer stack?
    fn has_outer_stack(&self) -> bool {
        match self.outer_stack {
            Some(_) => true,
            None => false
        }
    }

    fn while_function(&mut self, _object_and_scope: Pair<Value, Scope>) {
        //not implemented yet
    }

    // this function calls the topmost object on the stack as function
    fn call(&mut self, object_and_scope: Pair<Value, Scope>) {
        // create a new stackframe
        let mut s = StackFrame::new(
            Some(Box::new(self.clone())),
            object_and_scope.second,
            object_and_scope.first
        );

        // run the stackframe
        s.run();

        // pop off all of the arguments given to the function called
        for _ in 0..s.number_of_args_taken {
            self.pop();
        }

        // receive returned values
        s.contents.reverse();
        while !s.is_empty() {
            self.push(s.pop());
        }
    }

    // retrieve the value with a given variable name
    fn load(&mut self, name: String) -> Pair<Value, Scope> {
        self.scope.get(name)
    }

    // store a value under the given variable name
    fn store(&mut self, name: String, object: Pair<Value, Scope>) {
        self.scope.define(name, object)
    }

    // push an object with its saved scope onto the stack
    fn push(&mut self, object_and_scope: Pair<Value, Scope>) {
        self.contents.push(object_and_scope);
    }

    // push an object without a scope onto the stack (used for literals)
    fn push_value(&mut self, object: Value) {
        self.contents.push(Pair {
            first: object,
            second: self.scope.clone()
        });
    }

    // pop an object with its scope off of the stack or the parent stackframes' stack
    fn pop(&mut self) -> Pair<Value, Scope> {
        if self.is_empty() && self.has_outer_stack() {
            match &mut self.outer_stack {
                Some(s) => {
                    self.number_of_args_taken += 1;
                    s.pop()
                },
                None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
            }
        } else {

            let back: Pair<Value, Scope> = match self.contents.last() {
                Some(v) => v.clone(),
                None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
            };
            
            self.contents.pop();
            back.clone()
        }
    }

    // pop an object (a literal) off of the stack or the parent stackframes' stack
    fn pop_value(&mut self) -> Value {
        if self.is_empty() && self.has_outer_stack() {
            match &mut self.outer_stack {
                Some(s) => {
                    self.number_of_args_taken += 1;
                    s.pop().first
                },
                None => return Value::from_nothing()
            }
        } else {
            let back: Pair<Value, Scope> = self.contents.last().unwrap().clone();
            self.contents.pop();
            back.clone().first
        }
    }
}
