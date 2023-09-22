use crate::parser::Node;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

pub struct Runtime {
  functions: HashMap<String, Vec<Node>>, // mapping b/w name of fn & nodes in that fn 
  stack: Vec<HashMap<String, Value>>, // hashmap 
}

impl Runtime {
  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, node: &Node) -> Result<Value, String> {
    match node {
      Node::Program{children} => {
        for n in children {
          match n { // calls functions depending on node type thats matched 
            Node::FunctionDefine{..} => {
              self.run(n);
            },
            Node::IfElseStatements{..} => {
                self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
              },
            Node::ConditionalExpression{..} => {
                self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
              },
            Node::Expression{..} => {
              self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
            },
            Node::Statement{..} => {
              self.functions.insert("main".to_string(), vec![n.clone()]);
            }
            _ => (),
          }
        }
        Ok(Value::Bool(true))
      },

      // If the `Node` is a `MathExpression`, evaluate it.
      Node::MathExpression { name, children } => {
        // Evaluate the left and right children of the `MathExpression`.
        match (self.run(&children[0]), self.run(&children[1])) {
            // If both children are `Number` values, extract their values and evaluate the expression.
            (Ok(Value::Number(lhs)), Ok(Value::Number(rhs))) => {
                match name.as_ref() {
                    // If the operator is `+`, add the values.
                    "+" => Ok(Value::Number(lhs + rhs)),
                    // If the operator is `-`, subtract the values.
                    "-" => Ok(Value::Number(lhs - rhs)),
                    // If the operator is `*`, multiply the values.
                    "*" => Ok(Value::Number(lhs * rhs)),
                    // If the operator is `/`, divide the values.
                    "/" => Ok(Value::Number(lhs / rhs)),
                    // If the operator is `^`, raise the left value to the power of the right value.
                    "^" => {
                        let mut result = 1;
                        for i in 0..rhs {
                            result = result * lhs;
                        }
                        Ok(Value::Number(result))
                    },
                    // If the operator is not recognized, return an error message.
                    _ => Err("Undefined operator".to_string()),
                }
            }
            // If either child is not a `Number` value, return an error message.
            _ => Err("Cannot do math on String or Bool".to_string()),
        }
    },

       // If the `Node` is a `FunctionCall`, evaluate it.
       Node::FunctionCall { name, children } => {
        // Extract the input arguments.
        let in_args = if children.len() > 0 {
            match &children[0] {
                Node::FunctionArguments { children } => {
                    children
                },
                _ => children,
            }
        } else {
            children
        };
        // Create a new frame for local variables.
        let mut new_frame = HashMap::new();
       let mut val = Value::Bool(true); // initialises val to true

       // Save a raw pointer to the `Runtime` instance for use in the nested closure.
        let rt = self as *mut Runtime;
        // Find the named function and evaluate its body.
        match self.functions.get(name) {
            Some(statements) => {
                {
                    // If the function has input arguments, bind their values to the corresponding parameters.
                    match statements[0].clone() {
                        Node::FunctionArguments { children } => {
                            for (ix, arg) in children.iter().enumerate() {
                                // Use unsafe Rust code to call `run` on the input argument and handle any errors.
                                unsafe {
                                    let result = (*rt).run(&in_args[ix])?;
                                    match arg {
                                        Node::Expression { children } => {
                                            match &children[0] {
                                                Node::Identifier { value } => {
                                                    new_frame.insert(value.clone(), result);
                                                },
                                                _ => (),
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
                // Push the new frame onto the stack.
                self.stack.push(new_frame);
                // Evaluate each statement in the function body.
                for n in statements.clone() {
                   // result = self.run(&n);
                   val = self.run(&n)?;
                }
                // Pop the frame off the stack.
                self.stack.pop();
                return Ok(val) 
            },
            None => (),
        };
        // Return the result of evaluating the function.
       // result
       Err("Undefined function".to_string())
    },

      Node::ConditionalValue{children} => { 
        let lhs = match children[0] { // checks first argument in children is number, identfier, boolean or math expression for it to be valid conditional expression 
            Node::Number { .. } |
            Node::Identifier { .. } |
            Node::Bool { .. } |
            Node::MathExpression { .. } => {
                self.run(&children[0]) 
            },
            _ => Err("Unknown Statement".to_string()), // if none of those nodes match, prints error 
        };
          lhs
      },

      Node::ConditionalOperator{value} => { // returns the conditional operator value as a string 
        Ok(Value::String(value.clone()))
       },
      
    
      
    Node::ConditionalExpression{children} => {
         // Evaluates the left hand side & right hand side of conditional expressions 
       let lhs_val = match &children[0]{  // matches children[0] to ConditionalValue node 
            Node::ConditionalValue { .. } => {
                self.run(&children[0])
            },
            _ => Err("Unknown value".to_string()), // if doesn't match, returns error message 
        };

        let op_val = match &children[1]{ // matches children[1] to ConditionalOperator node 
            Node::ConditionalOperator { .. } => {
                self.run(&children[1])
            },
            _ => Err("Unknown operator".to_string()), // if doesn't match, returns error message 
        };
        let rhs_val = match &children[2]{ // matches children[2] to ConditionalValue node 
            Node::ConditionalValue { .. } => {
                self.run(&children[2])
            },
            _ => Err("Unknown value".to_string()), // if doesn't match, returns error message 
        };

        let string_op_val = match op_val.unwrap(){ // unwraps the returned op_val to a string 
            Value::String(value) => {
                value
            }, 
            _ => ("Operator error".to_string())
        };

        let mut lhs_bool = false;
        let mut rhs_number = false;
        let mut lhs_number = false;
        let mut rhs_bool = false;

        // checks if lhs argument is a boolean or a number 
        let check_lhs = match lhs_val.clone().unwrap(){
            Value::Bool(lhs_val) => {
                lhs_bool = true;
            },
            Value::Number(lhs) => {
                lhs_number = true;
            },
            _ => (),
        };

        // checks if rhs argument is a boolean or a number 
        let check_rhs = match rhs_val.clone().unwrap(){
            Value::Bool(rhs_val) =>{
                rhs_bool = true;
            }, 
            Value::Number(rhs_val) =>{
                rhs_number = true;
            }, 
            _ => (),
        };

        // if lhs is a boolean & rhs is a number or rhs is a boolean & lhs is a number, returns error message
        // cannot compare boolean and numbers 
        if((lhs_bool && rhs_number) || (rhs_bool && lhs_number)){
            return Err("Cannot compare these two values".to_string())
        } 

        // depending on the operator value, performs conditional operations on lhs & rhs and returns the result as a boolean
        if (string_op_val == "<"){
            Ok(Value::Bool(lhs_val < rhs_val))
        } else if (string_op_val == ">"){
            Ok(Value::Bool(lhs_val > rhs_val))
        } else if (string_op_val == "=="){
            Ok(Value::Bool(lhs_val == rhs_val))
        } else if (string_op_val == "!="){
            Ok(Value::Bool(lhs_val != rhs_val))
        } else if (string_op_val == ">="){
            Ok(Value::Bool(lhs_val >= rhs_val))
        } else if (string_op_val == "<="){
            Ok(Value::Bool(lhs_val <= rhs_val))
        } else {
            Err("Unknown operator".to_string()) // if none of these operators match, returns unkown operator error message
        }
    },

      // Defines a new function based on the elements in the children argument. The name of the function is retrieved from the first element of the children, and the statements that define the function are retrieved from rest of hte children (head/tail). A new key-value pair is then inserted into the functions field of the current runtime object. If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{children} => { 
        let (head, tail) = children.split_at(1);
        match &head[0] {
            Node::Identifier { value } => {
                self.functions.insert(value.to_string(), tail.to_vec());
            },
            _ => (),
        }
        Ok(Value::Bool(true))
      },

       Node::IfElseStatements{children} => { 
        // loops through all elements in children and accordingly matches to respective nodes and calls run function  
        for c in children{ 
            match c {
                Node::IfStatement { .. } => {
                    self.run(&children[0])
                },
                Node::ElseIfStatement { .. } => {
                    self.run(&children[0])
                },
                Node::ElseStatement { .. } => {
                    self.run(&children[0])
                },
                _ => Err("Unknown Statement".to_string()), // if none match, returns error message 
            };

        }
          return Ok(Value::Bool(true))
      },

      // this node contains a conditional if statement and statements that need to be executed if condition is met
      Node::IfStatement{children} => { 
        let if_stat_cond = match children[0] { // matches children[0] to a ConditionalExpression 
            Node::ConditionalExpression { .. } => {
                self.run(&children[0])
            },
            _ => Err("Unknown Expression".to_string()), // returns error message if don't match 
        };

        let mut condition = false;
        match &if_stat_cond.clone().unwrap() { // unwraps the result of the ConditionalExpression
            Value::Bool (value) => {
                condition = *value;
            },
            _ => (),
        }

        // if the condition was met, executes the statements inside the if block 
        if(condition){
            for c in children{
                let result = match c { // matches the children to Statement node 
                    Node::Statement { .. } => {
                        self.run(&c)
                    },
                    _ => Err("Unknown Statement".to_string()), // returns error message if don't match 
                };    
            }
        } 
        if_stat_cond
      },

      // this node contains a conditional else if statement and statements that need to be executed if condition is met
      Node::ElseIfStatement{children} => { 
        let else_if_stat_cond = match children[0] { // matches children[0] to a ConditionalExpression 
            Node::ConditionalExpression { .. } => {
                self.run(&children[0])
            },
            _ => Err("Unknown Statement".to_string()), // returns error message if don't match 
        };

        let mut condition = false;
        match &else_if_stat_cond.clone().unwrap() { // unwraps the result of the ConditionalExpression
            Value::Bool (value) => {
                condition = *value;
            },
            _ => (),
        }

        // if the condition was met, executes the statements inside the else if block 
        if(condition){
            for c in children{
                let result = match c {  // matches each element to Statement node 
                    Node::Statement { .. } => {
                        self.run(&c)
                    },
                    _ => Err("Unknown Statement".to_string()), // returns error message if don't match 
                };    
            }
         }
        else_if_stat_cond
      },


      // this node contains statements that need to be executed 
      Node::ElseStatement{children} => { 
        for c in children{
            let result = match c { // matches each element to Statement node 
                Node::Statement { .. } => {
                    self.run(&c)
                },
                _ => Err("Unknown Statement".to_string()), // returns error message if don't match 
            };    
        }
        Ok(Value::Bool(true))
      },

      // Calls the run method on the first element in the children argument, which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        self.run(&children[0]) // recursively calls run method on elements in children 
      },

      // citation: HW 5 solutions interpreter.rs file = looked at Identifier for reference
       // If the `Node` is an `Identifier`, look up its value in the current frame.
       Node::Identifier { value } => {
        let last = self.stack.len() - 1;
        match self.stack[last].get(value) {
            Some(id_value) => Ok(id_value.clone()),
            None => Err("Undefined variable".to_string()),
        }
    },


      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => { 
        match children[0] { // if children[0] matches VariableDefine or FunctionReturn, runs associated function 
          Node::VariableDefine { .. } |
          Node::FunctionReturn { .. } => {
              self.run(&children[0])
          },
          _ => Err("Unknown Statement".to_string()), // returns error message if doesn't match 
      }
      },

       // If the `Node` is a `VariableDefine`, evaluate its expression and bind the result to a new variable.
       Node::VariableDefine { children } => {
        // Extract the variable name.
        let name: String = match &children[0] {
            Node::Identifier { value } => value.clone(),
            _ => "".to_string(),
        };
        // Evaluate the expression.
        let value = self.run(&children[1])?;
        // Add the variable to the current frame.
        let last = self.stack.len() - 1;
        self.stack[last].insert(name, value.clone());
        // Return the value.
        Ok(value)
    }

       Node::Expression { children } => {
        match children[0] { // if children[0] matches MathExpression, Number, FunctionCall, String, Bool, or Identifier, runs the associated function
            Node::MathExpression { .. } |
            Node::Number { .. } |
            Node::FunctionCall { .. } |
            Node::String { .. } |
            Node::Bool { .. } |
            Node::Identifier { .. } => {
                self.run(&children[0])
            },
            _ => Err("Unknown Expression".to_string()), // returns error message if doesn't match 
        }
        }

      Node::Number{value} => { 
        Ok(Value::Number(*value)) // returns val assigned to that number 
      }
    
      Node::String{value} => { 
       Ok(Value::String(value.clone())) // returns string val 
      }

      Node::Bool{value} => {
        Ok(Value::Bool(*value)) // returns bool val
      }
       // If the `Node` is of an unhandled type, return an error message.
       _ => {
        Err("Unhandled Node".to_string())
        },
    }
  }

}

pub fn start_interpreter(node: &Node) -> Result<Value, String> {  
  let mut runtime = Runtime::new();
  runtime.run(node);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}




