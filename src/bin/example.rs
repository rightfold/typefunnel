extern crate typefunnel;

use std::io;
use typefunnel::asset::serialization;
use typefunnel::source::HasSchema;
use typefunnel::source::call::ECMAScript;
use typefunnel::source::constant::Constant;

fn main() {
  safe_main().unwrap();
}

fn safe_main() -> io::Result<()> {
  let source = Constant::AllOf(vec![
    Constant::SignedInteger(42),
    Constant::DoublePrecision(3.14),
    Constant::AllOf(vec![
      Constant::String("Hello, world!".to_string()),
      Constant::String("Bye, world!".to_string()),
    ]),
  ]);
  let (_, schema) = source.schema()?;
  println!("// serialization");
  serialization::ecmascript::serialize(&mut io::stdout(), &schema)?;
  println!("\n// call");
  source.ecmascript_call(&mut io::stdout())?;
  Ok(())
}
