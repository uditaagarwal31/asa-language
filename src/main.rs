extern crate nom;
extern crate asalang;

use asalang::{program, Runtime, Node, start_interpreter};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"8<9"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      let interpreter_result = start_interpreter(&tree);
      println!("{:?}", interpreter_result);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }
  Ok(())
}

