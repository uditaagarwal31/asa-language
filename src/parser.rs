// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.
use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alphanumeric1, digit1},
  };
  // Here are the different node types used in parser and grammar
  #[derive(Debug, Clone)]
  pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    FunctionReturn { children: Vec<Node> },
    FunctionDefine { children: Vec<Node> },
    FunctionArguments { children: Vec<Node> },
    FunctionStatements { children: Vec<Node> },
    Expression { children: Vec<Node> },
    MathExpression {name: String, children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    ConditionalOperator { value: String },
    ConditionalValue { children: Vec<Node> },
    ConditionalExpression { children: Vec<Node> },
    ConditionalExpressionMultiple { children: Vec<Node> },
    IfStatement{ children: Vec<Node> },
    ElseStatement{ children: Vec<Node> },
    ElseIfStatement{ children: Vec<Node> },
    IfElseStatements{ children: Vec<Node> },

  }
  
  // identifier = {alnum} ;
  pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
    Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
  }
  
  // number = {digit} ;
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
  }

  // boolean  = "true" | "false" ;
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?; // takes true and false as a list of tags and returns whichever tag the parser recognizes in input 
    let bool_value = if result == "true" {true} else {false}; // uses match statement to return the boolean output depending on tag 
    Ok((input, Node::Bool{ value: bool_value}))
  }

  // string  = "\"" , {alnum | " "} , "\"" ;
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?; // tag recognizes "\" and consumes it and returns partially consumed input in input
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?; // applies parser 1 or more times while applies alt to check for alphanumeric characters or space " " tag and if there's alphanum or space it consumes that and returns the now partially consumed input in input
    let (input, _) = tag("\"")(input)?; // tag recognizes "\" and consumes it and returns partially consumed input in input
    Ok((input, Node::String{ value: string.join("")})) // Return the now partially consumed input, as well as a node with the string on it.
  }

  // function_call  = identifier , "(" , [arguments] , ")" ;
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = alphanumeric1(input)?; // Consumes at least 1 alphanumeric character and returns in name 
    let (input, _) = tag("(")(input)?; // tag recognizes "(" and consumes it and returns partially consumed input in input
    let (input, mut args) = many0(arguments)(input)?; // applies parser 0 or more times to recognise arguments function and returns in args 
    let (input, _) = tag(")")(input)?; // tag recognizes ")" and consumes it and returns partially consumed input in input
    Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
  }

  // parenthetical_expression = "(" , l1, ")" ;
  pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag("(")(input)?;  // tag recognizes "(" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, args) = l1(input)?; // calls l1 function which returns the output of the function in args as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag(")")(input)?;  // tag recognizes ")" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    Ok((input, args))
  }

  // l4 = (function_call | number | identifier | parenthetical_expression) ;
  pub fn l4(input: &str) -> IResult<&str, Node> {
    alt((function_call, number, identifier, parenthetical_expression))(input) // takes function_call, number, identifier, parenthetical_expression as a list of functions and returns whichever function the parser recognizes in input 
  }

  // l3_infix = "^", l4 ; 
  pub fn l3_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, op) = tag("^")(input)?;  // tag recognizes "^" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, args) = l4(input)?; // calls l4 function which returns the output of the function in args as well as the now partially consumed input in input
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }

  // l3 = l4, [l3_infix] ; 
  pub fn l3(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l4(input)?; // calls l4 function which returns the output of the function in args as well as the now partially consumed input in input
    let (input, tail) = many0(l3_infix)(input)?; // applies parser 0 or more times to recognise l3_infix function and returns in tail 
    for n in tail { // loops through each element n in tail and matches to math expression node
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }

  // l2_infix = ("*" | "/"), l2 ;
  pub fn l2_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, op) = alt((tag("*"),tag("/")))(input)?; // takes * and / as a list of tags and returns whichever tag the parser recognizes in input 
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, args) = l2(input)?; // calls l2 function which returns the output of the function in args as well as the now partially consumed input in input
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }

  // l2 = l3, [l2_infix] ; 
  pub fn l2(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l3(input)?; // calls l3 function which returns the output of the function in head as well as the now partially consumed input in input
    let (input, tail) = many0(l2_infix)(input)?; // applies parser 0 or more times to recognise l2_infix function and returns in tail 
    for n in tail { // loops through each element n in tail and matches to math expression node
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }

  // l1_infix = ("+" | "-"), l2 ;
  pub fn l1_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, op) = alt((tag("+"),tag("-")))(input)?; // takes + and - as a list of tags and returns whichever tag the parser recognizes in input 
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, args) = l2(input)?; // calls l2 function which returns the output of the function in args as well as the now partially consumed input in input
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }

  // l1 = l2, [l1_infix] ;
  pub fn l1(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l2(input)?; // calls l2 function which returns the output of the function in head as well as the now partially consumed input in input
    let (input, tail) = many0(l1_infix)(input)?;  // applies parser 0 or more times to recognise l1_infix function and returns in tail 
    for n in tail { // loops through each element n in tail and matches to math expression node
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }

  // math_expression = l1 ; 
  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    l1(input) // calls l1 function 
  }

  // expression = boolean | math_expression | function_call | string ;
  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((boolean, math_expression, function_call, string))(input)?; // takes boolean, math_expression, function_call, string as a list of functions and returns whichever function the parser recognizes in input
    Ok((input, Node::Expression{ children: vec![result]}))   
  }

  // statement  = variable_define , ";" ;
  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?; // applies parser 0 or more times to recognise space or tab tags 
    let (input, result) = variable_define(input)?; // calls variable_define function which returns the output of the function in result as well as the now partially consumed input in input
    let (input, _) = tag(";")(input)?; // tag recognizes ";" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    Ok((input, Node::Statement{ children: vec![result]}))   
  }

  // function_return = "return", (function_call | expression | identifier) ;
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?; // tag recognizes "return " and consumes it and returns partially consumed input in input
    let (input, return_value) = alt((function_call, expression, identifier))(input)?; // takes function_call, expression, identifier as a list of functions and returns whichever function the parser recognizes in input
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
  }

  // variable_define = "let" , identifier , "=" , expression ;
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let ")(input)?; // tag recognizes "let " and consumes it and returns partially consumed input in input
    let (input, variable) = identifier(input)?; // calls identifier function which returns the output of the function in variable as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag("=")(input)?; // tag recognizes "=" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, expression) = expression(input)?; // calls expression function which returns the output of the function in expression as well as the now partially consumed input in input
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
  }

//   if = "if", conditional_ex, "{", [statement], function_return, "}" ; 
pub fn if_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("if ")(input)?; // tag recognizes "if " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?;  // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, conditional_ex) = conditional_exp(input)?; // calls conditional_exp function which returns the output of the function in conditional_ex as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?;  // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?;  // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag("{")(input)?; // tag recognizes "{" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?;  // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, mut statements) = many0(statement)(input)?; // many0 applies parser 0 or more times to call statement function which returns the output of the function in statements as well as the now partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, mut return_stat) = many1(function_return)(input)?;  // many1 applies parser 1 or more times to call function_return function which returns the output of the function in return_stat as well as the now partially consumed input in input
    let (input, _) = tag(";")(input)?; // tag recognizes "; " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, _) = tag("}")(input)?; // tag recognizes "}" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let mut children_temp = vec![conditional_ex.clone()]; 
    children_temp.append(&mut statements); // adds statements to children_temp vector 
    children_temp.append(&mut return_stat); // adds return_stat to children_temp vector 
   Ok((input, Node::IfStatement{ children: children_temp}))  
}

// else =  "else", "{", [statement], function_return, "}" ;
pub fn else_statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else ")(input)?; // tag recognizes "else " and consumes it and returns partially consumed input in input
    let (input, _) = tag("{")(input)?; // tag recognizes "{" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?;  // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, mut statements) = many0(statement)(input)?; // many0 applies parser 0 or more times to call statement function which returns the output of the function in statements as well as the now partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, mut return_stat) = many1(function_return)(input)?; // many1 applies parser 1 or more times to call function_return function which returns the output of the function in return_stat as well as the now partially consumed input in input
    let (input, _) = tag(";")(input)?; // tag recognizes "; " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, _) = tag("}")(input)?; // tag recognizes "} " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let mut children_temp = vec![];
    children_temp.append(&mut statements); // adds statements to children_temp vector 
    children_temp.append(&mut return_stat); // adds return_stat to children_temp vector 
    return Ok((input, Node::ElseStatement{ children: children_temp}))
}

// else_if = "else if", conditional_ex, "{", [statement], function_return, "}" ;
pub fn else_if(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else if ")(input)?; // tag recognizes "else if " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, conditional_ex) = conditional_exp(input)?; // calls conditional_exp function which returns the output of the function in conditional_ex as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag("{")(input)?; // tag recognizes "{" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, mut statements) = many0(statement)(input)?; // many0 applies parser 0 or more times to call statement function which returns the output of the function in statements as well as the now partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, mut return_stat) = many1(function_return)(input)?; // many1 applies parser 1 or more times to call function_return function which returns the output of the function in return_stat as well as the now partially consumed input in input
    let (input, _) = tag(";")(input)?; // tag recognizes "; " and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let (input, _) = tag("}")(input)?; // tag recognizes "}" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = many0(tag("\t"))(input)?; // many0 applies parser 0 or more times to check for tab "\t" and if there is a tab, it consumes that tab and returns the now partially consumed input in input
    let mut children_temp = vec![conditional_ex.clone()];
    children_temp.append(&mut statements); // adds statements to children_temp vector 
    children_temp.append(&mut return_stat); // adds return_stat to children_temp vector 
    return Ok((input, Node::ElseIfStatement{ children: children_temp}))
}

// if_else = if, [else_if], else ;
pub fn if_else_statements(input: &str) -> IResult<&str, Node> {
    let (input, if_stat) = if_statement(input)?; // calls if_statement function which returns the output of the function in if_stat as well as the now partially consumed input in input
    let (input, mut else_if_stat) = many0(else_if)(input)?; // many0 applies parser 0 or more times to call else_if function which returns the output of the function in else_if_stat as well as the now partially consumed input in input
    let (input, mut else_stat) = many1(else_statement)(input)?; // many1 applies parser 1 or more times to call else_statement function which returns the output of the function in else_stat as well as the now partially consumed input in input
    let mut children_temp = vec![if_stat.clone()];
    children_temp.append(&mut else_if_stat); // adds else_if_stat to children_temp vector 
    children_temp.append(&mut else_stat); // adds else_stat to children_temp vector 
    return Ok((input, Node::IfElseStatements{ children: children_temp}))
}


// conditional_exp = conditional_val, conditional_operator, conditional_val, [conditional_operator, conditional_val] ;
pub fn conditional_exp(input: &str) -> IResult<&str, Node> {
    let (input, conditional_val1) = conditional_val(input)?; // calls conditional_val function which returns the output of the function in conditional_val1 as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, conditional_op1) = conditional_operator(input)?; // calls conditional_operator function which returns the output of the function in conditional_op1 as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, conditional_val2) = conditional_val(input)?; // calls conditional_val function which returns the output of the function in conditional_val2 as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, mut conditional_op2) = many0(conditional_operator)(input)?; // many0 applies parser 0 or more times to call conditional_operator function which returns the output of the function in conditional_op2 as well as the now partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, mut conditional_val3) = many0(conditional_val)(input)?; // many0 applies parser 0 or more times to call conditional_val function which returns the output of the function in conditional_val3 as well as the now partially consumed input in input
    let mut children = vec![conditional_val1, conditional_op1, conditional_val2];
    children.append(&mut conditional_op2); // adds conditional_op2 to children_temp vector 
    children.append(&mut conditional_val3); // adds conditional_val3 to children_temp vector 
    Ok((input, Node::ConditionalExpression{ children: children})) 
}

// conditional_operator      = "<" | ">" | "<=" | ">=" | "==" | "!=" ;
pub fn conditional_operator(input: &str) -> IResult<&str, Node> {
    let (input, conditional_op) = alt((tag("<"), tag(">"), tag("<="), tag(">="), tag("=="), tag("!=")))(input)?; // takes <, >, <=, >=, ==, != as a list of tags and returns whichever tag the parser recognizes in input 
    Ok((input, Node::ConditionalOperator{ value: conditional_op.to_string()})) // Return the now partially consumed input, as well as a node with the string on it
}

// conditional_val = number | boolean | identifier | math_expression ;
pub fn conditional_val(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((boolean, math_expression, number, identifier))(input)?; // takes boolean, math_expression, number, identifier as a list of functions and returns whichever function the parser recognizes in input 
    Ok((input, Node::ConditionalValue{ children: vec![result]}))   
}

// arguments  = expression , [other_arg] ;
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, arg) = expression(input)?; // calls expression function which returns the output of the function in arg as well as the now partially consumed input in input
    let (input, mut others) = many0(other_arg)(input)?; // many0 applies parser 0 or more times to call other_arg function which returns the output of the function in others as well as the now partially consumed input in input
    let mut args = vec![arg];
    args.append(&mut others); // adds others to args vector 
    Ok((input, Node::FunctionArguments{children: args}))
  }

  // other_arg = ",", expression ; 
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?; // tag recognizes "," and consumes it and returns partially consumed input in input
    expression(input)
  }

  // function_definition  = "fn" , identifier , "(" , [arguments] , ")" , "{" , {statement} , "}" 
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("fn ")(input)?; // tag recognizes "fn" and consumes it and returns partially consumed input in input
    let (input, function_name) = identifier(input)?;  // calls identifier function which returns the output of the function in function_name as well as the now partially consumed input in input
    let (input, _) = tag("(")(input)?; // tag recognizes "(" and consumes it and returns partially consumed input in input
    let (input, mut args) = many0(arguments)(input)?; // many0 applies parser 0 or more times to call arguments function which returns the output of the function in args as well as the now partially consumed input in input
    let (input, _) = tag(")")(input)?; // tag recognizes ")" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag(" "))(input)?; // many0 applies parser 0 or more times to check for space " " and if there is a space, it consumes that space and returns the now partially consumed input in input
    let (input, _) = tag("{")(input)?; // tag recognizes "{" and consumes it and returns partially consumed input in input
    let (input, _) = many0(tag("\n"))(input)?; // many0 applies parser 0 or more times to check for newline "\n" and if there is a newline, it consumes that newline and returns the now partially consumed input in input
    let (input, mut statements) = many1(statement)(input)?; // many1 applies parser 1 or more times to call statement function which returns the output of the function in statements as well as the now partially consumed input in input
    let (input, _) = tag("}")(input)?; // tag recognizes "}" and consumes it and returns partially consumed input in input
    let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?; // many0 applies parser 0 or more times to check for newline "\n" or " "and if either are there it consumes that and returns the now partially consumed input in input
    let mut children = vec![function_name];
    println!("args, {:?}", args);
    children.append(&mut args); // appends args in children vector 
    children.append(&mut statements);  // appends statements in children vector 
    Ok((input, Node::FunctionDefine{ children: children }))   
  }

  // program = {function_definition | if_else_statements | function_call | statement | variable_define | conditional_exp | expression} ;
  pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many1(alt((function_definition, if_else_statements, function_call, statement, variable_define, conditional_exp, expression)))(input)?;  // many1 applies parser 1 or more times to take a list of functions function_definition, if_else_statements, function_call, statement, variable_define, conditional_exp, expression and returns it to result
    Ok((input, Node::Program{ children: result}))   
  }
  