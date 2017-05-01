extern crate typefunnel;

use std::io;
use typefunnel::asset::web_service;
use typefunnel::edit_warning;
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
  println!("{}", edit_warning::ECMASCRIPT);
  web_service::ecmascript::handle(&mut io::stdout(), &source)?;
  Ok(())
}
