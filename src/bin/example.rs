extern crate typefunnel;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
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
  let source = Constant::AllOf(vec![
    Constant::SignedInteger(42),
    Constant::DoublePrecision(3.14),
    Constant::AllOf(vec![
      Constant::String("Hello, world!".to_string()),
      Constant::String("Bye, world!".to_string()),
    ]),
  ]);

  fs::create_dir_all("/tmp/typefunnel")?;
  generate_server(&source)?;
  generate_client(&source)?;

  Ok(())
}

fn generate_server<Source>(source: Source) -> io::Result<()>
  where Source: HasSchema + ECMAScript + Copy {
  let mut file = File::create("/tmp/typefunnel/server.js")?;
  write!(file, "{}\n", edit_warning::ECMASCRIPT)?;
  write!(file, "var express = require('express');\n")?;
  write!(file, "var bodyParser = require('body-parser');\n")?;
  write!(file, "var app = express();\n")?;
  write!(file, "app.use(bodyParser.json({{strict: false}}));\n")?;
  write!(file, "app.post('/', ")?;
  web_service::ecmascript::handle(&mut file, source)?;
  write!(file, ".bind(null, null));\n")?;
  write!(file, "app.listen(1337);\n")?;
  Ok(())
}

fn generate_client<Source>(source: Source) -> io::Result<()>
  where Source: HasSchema {
  let mut file = File::create("/tmp/typefunnel/client.js")?;
  let (input_schema, output_schema) = source.schema()?;
  let client = WebService(input_schema, output_schema);
  writeln!(file, "{}", edit_warning::ECMASCRIPT)?;
  client.ecmascript_call(&mut file)?;
  Ok(())
}
