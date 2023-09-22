extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, Runtime};
use asalang::interpreter::start_interpreter;
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          let mut runtime = Runtime::new();
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected); 
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}



test!(numeric, r#"123"#, Ok(Value::Number(123))); 
test!(identifier, r#"x"#, Err("Undefined variable".to_string())); 
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string()))); 
test!(bool_true, r#"true"#, Ok(Value::Bool(true))); 
test!(bool_false, r#"false"#, Ok(Value::Bool(false))); 
test!(function_call, r#"foo()"#, Err("Undefined function".to_string())); 
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function".to_string())); 
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function".to_string())); 
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123))); 
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1))); 
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true))); 
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1))); 
test!(math, r#"1 + 1"#, Ok(Value::Number(2))); 
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2))); 
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0))); 
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8))); 
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));  
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16))); 
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22))); 
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18))); 
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function".to_string())); 
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function".to_string())); 
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5))); 
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6))); 
test!(define_function_more_statement, r#"fn main() { 
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));
// added tests as part of the deliverable
test!(string_new, r#""hi""#, Ok(Value::String("hi".to_string()))); // tests single word strings 
test!(math_new, r#"7 + 7"#, Ok(Value::Number(14))); // tests addition
test!(variable_def_new, r#"let z = 99;"#, Ok(Value::Number(99))); // tests variable define
test!(math_exponent_new, r#"3 ^ 2"#, Ok(Value::Number(9))); // tests exponents
test!(conditional_ex1, r#"5 < 7"#, Ok(Value::Bool(true))); 
test!(conditional_ex2, r#"11 + 6 * 2 < 5 * 2 - 3"#, Ok(Value::Bool(false))); 
test!(conditional_ex3, r#"7 > true"#, Err("Cannot compare these two values".to_string())); 
test!(if_else_ex1, r#"if 1 < 2 {
  let x = 9;
  return true;
  } else if 3 == 2 {
  return false;
  } else {
      return true;
  }
  "#, Ok(Value::Bool(true))); 
  test!(if_else_ex2, r#"if 1+7 > 9 {
    return true;
    } else if 9 != 2 {
    return true;
    } else {
        return false;
    }
    "#, Ok(Value::Bool(true))); 
    test!(if_else_ex3, r#"if 4 > 3 {return true;} else if 7 == 9 {return false;} else {return true;}
      "#, Ok(Value::Bool(true))); 



