extern crate typefunnel;

use std::io;
use typefunnel::asset::web_service;
use typefunnel::edit_warning;
use typefunnel::source::HasSchema;
use typefunnel::source::call::ECMAScript;
use typefunnel::source::constant::Constant;
use typefunnel::source::web_service::WebService;

fn main() {
  safe_main().unwrap();
}

fn safe_main() -> io::Result<()> {
  let server = Constant::AllOf(vec![
    Constant::SignedInteger(42),
    Constant::DoublePrecision(3.14),
    Constant::AllOf(vec![
      Constant::String("Hello, world!".to_string()),
      Constant::String("Bye, world!".to_string()),
    ]),
  ]);
  println!("{}", edit_warning::ECMASCRIPT);
  web_service::ecmascript::handle(&mut io::stdout(), &server)?;

  let (input_schema, output_schema) = server.schema()?;
  let client = WebService(input_schema, output_schema);
  println!("{}", edit_warning::ECMASCRIPT);
  client.ecmascript_call(&mut io::stdout())?;

  Ok(())
}
