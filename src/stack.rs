use crate::error::*;
use crate::object::*;
use crate::value::*;
use crate::literals::*;
use crate::table::Table;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair<A, B> {
    first: A,
    second: B
}

#[derive(Debug, Clone, PartialEq)]
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
        let value_scope = match self.table.get(name.clone()) {
            Some(v) => return v,
            None => {
                // the value isnt in this scope
                // try to use the parent scope if possible, 
                // otherwise just return an empty value and scope
                match &mut self.outer_scope {
                    Some(v) => (*v).get(name),
                    None => Pair{first: Value::from_nothing(), second: Scope::new(None)}
                }
            }
        };

        match value_scope.first.get_type() {
            Type::Function => value_scope,
            _ => Pair{first: value_scope.first, second: Scope::new(None)}
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
        // println!("stack {:?}", match self.outer_stack.clone().contents{
        //     Some
        // });
        for instruction in self.instructions.as_list() {
            self.step(instruction)
        }
    }

    fn step(&mut self, instruction: Value) {
        match instruction.as_instruction() {
            // print the topmost object without a carriage return
            Instruction::Print => self.pop_value().print(),
            // print the topmost object with a carriage return
            Instruction::Println => self.pop_value().println(),

            // call the topmost object on the stack as
            // a function (as if it were in this scope)
            // until it returns 0
            Instruction::While => {
                let condition = self.pop_value();
                let body = self.pop_value();
                self.while_function(condition, body);
            },

            Instruction::If => {
                // println!("{:#?}", self.contents);
                let c = self.pop_value();
                let a = self.pop();
                let b = self.pop();
                // println!("{} {} {}", c, a.first, b.first);
                if c != num("0") && c != none() {
                    self.push(a);
                } else {
                    self.push(b);
                }
            },

            Instruction::Append => {
                let mut list = self.pop_value();
                let value = self.pop_value();
                list.list_push(value);
                self.push_value(list);
            },

            Instruction::Pop => {
                let mut list = self.pop_value();
                let value = list.list_pop();
                self.push_value(value);
            },
            
            Instruction::Index => {
                let mut list = self.pop_value();
                let index = self.pop_value();
                self.push_value(list.index(index));
            },

            // == the topmost objects
            Instruction::Equal => {
                let a = self.pop_value();
                let b = self.pop_value();
                if a == b {
                    self.push_value(num("1"));
                } else {
                    self.push_value(num("0"));
                }
            },

            // > the topmost objects
            Instruction::Greater => {
                let a = self.pop_value();
                let b = self.pop_value();
                if a.as_number() > b.as_number() {
                    self.push_value(num("1"));
                } else {
                    self.push_value(num("0"));
                }
            },
            
            // < the topmost objects
            Instruction::Less => {
                let a = self.pop_value();
                let b = self.pop_value();
                if a.as_number() < b.as_number() {
                    self.push_value(num("1"));
                } else {
                    self.push_value(num("0"));
                }
            },
            

            // not the topmost object
            Instruction::Not => {
                let a = self.pop_value();
                self.push_value(!a);
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
                // println!("get attr");
                let mut names: Vec<String> = vec![];
                // println!("contents {:?}", self.contents.len());
                loop {

                    if !self.contents.iter().any(|v| v.first.get_type() == Type::Instance) {
                        throw("No instance to set attribute of", self.contents.clone());
                    }

                    let back = match self.contents.last() {
                        Some(k) => k.first.clone(),
                        None => {
                            throw("Could not get back item from stack", self.contents.clone());
                            Value::from_nothing()
                        }
                    };

                    if back.get_type() == Type::Instance {
                        break;
                    }

                    names.push(
                        self.pop_value().as_string()
                        );
                    
                    if self.contents.len() < 1 {
                        throw("Too few items on stack to set attribute", self.contents.clone());
                    }
                }
                if names.len() < 1 {
                    throw("Could not set attribute of object without the attribute name", self.contents.clone());
                }
                names.reverse();

                let mut object = self.pop_value();

                self.push_value(object.get_attr_recursive(names));
            },

            // setattr modifies an attribute of an object
            Instruction::SetAttr => {
                // println!("set attr");
                let mut names: Vec<String> = vec![];
                // println!("contents {:?}", self.contents.len());
                loop {
                    if !self.contents.iter().any(|v| v.first.get_type() == Type::Instance) {
                        throw("No instance to set attribute of", self.contents.clone());
                    }

                    let back = match self.contents.last() {
                        Some(k) => k.first.clone(),
                        None => {
                            throw("Could not get back item from stack", self.contents.clone());
                            Value::from_nothing()
                        }
                    };

                    if back.get_type() == Type::Instance {
                        break;
                    }

                    names.push(
                        self.pop_value().as_string()
                        );
                    
                    if self.contents.len() < 2 {
                        throw("Too few items on stack to set attribute", self.contents.clone());
                    }
                }
                if names.len() < 1 {
                    throw("Could not set attribute of object without the attribute name", self.contents.clone());
                }

                names.reverse();

                let mut object = self.pop_value();
                let data = self.pop_value();


                self.push_value(object.set_attr_recursive(names, data));
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

    fn while_function(&mut self, condition: Value, body: Value) {
        loop {
            for instruction in condition.as_list() {
                self.step(instruction)
            }

            let result = self.pop_value(); 
            if result == num("0") || result == none() {
                break;
            }

            for instruction in body.as_list() {
                self.step(instruction)
            }
        }
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
            // println!("popped arg {:?}", self.pop().first);
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
        let result = self.scope.get(name.clone());
        let empty = Pair{first: Value::from_nothing(), second: Scope::new(None)};
        if result.first == empty.first && result.second == empty.second {
            return match &mut self.outer_stack {
                Some(s) => {
                    s.load(name.clone())
                },
                None => {
                    let v = self.pop();
                    self.store(name.clone(), v);
                    self.load(name.clone())
                }
            };
        } else {
            return result;
        }
    }

    // store a value under the given variable name
    fn store(&mut self, name: String, object: Pair<Value, Scope>) {
        if object.second != self.scope {
            self.scope.define(name, object);
        } else {
            self.scope.define(name, Pair{first: object.first, second: Scope::new(None)});
        }
        // match object.first.get_type() {
        //     Type::Function => self.scope.define(name, object),
        //     _ => self.scope.define(name, Pair{first: object.first, second: Scope::new(None)}),
        // }
    }

    // push an object with its saved scope onto the stack
    fn push(&mut self, object_and_scope: Pair<Value, Scope>) {
        // self.number_of_args_taken += 1;
        self.contents.push(object_and_scope);
    }

    // push an object without a scope onto the stack (used for literals)
    fn push_value(&mut self, object: Value) {
        // self.number_of_args_taken += 1;
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
                None => {
                    throw("Could not pop off of stack", self.contents.clone());
                    return Pair{first: Value::from_nothing(), second: Scope::new(None)}
                }
            }
        } else {

            let back: Pair<Value, Scope> = match self.contents.last() {
                Some(v) => v.clone(),
                None => {
                    throw("Could not pop off of stack", self.contents.clone());
                    Pair{first: Value::from_nothing(), second: Scope::new(None)}
                }
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
                None => {
                    throw("Could not pop off of stack", self.contents.clone());
                    return Value::from_nothing();
                }
            }
        } else {
            let back: Value = match self.contents.last() {
                Some(v) => v.first.clone(),
                None => {
                    throw("Could not pop off of stack", self.contents.clone());
                    Value::from_nothing()
                }
            };
            
            self.contents.pop();
            back.clone()
        }
    }
}
