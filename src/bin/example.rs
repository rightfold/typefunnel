extern crate typefunnel;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use typefunnel::asset::web_service;
use typefunnel::edit_warning;
use typefunnel::source::HasSchema;
use typefunnel::source::call::{ECMAScript, ECMAScriptModule};
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

fn generate_server<Source>(source: &Source) -> io::Result<()>
  where Source: HasSchema + ECMAScript {
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

fn generate_client<Source>(source: &Source) -> io::Result<()>
  where Source: HasSchema {
  let (input_schema, output_schema) = source.schema()?;
  let client = WebService(input_schema, output_schema);

  let module = ECMAScriptModule{
    calls: {
      let mut calls = HashMap::new();
      calls.insert("service".to_string(), (&client as &HasSchema, &client as &ECMAScript));
      calls
    },
  };

  {
    let mut file = File::create("/tmp/typefunnel/client.js")?;
    writeln!(file, "{}", edit_warning::ECMASCRIPT)?;
    module.ecmascript(&mut file)?;
  }

  {
    let mut file = File::create("/tmp/typefunnel/client.purs")?;
    writeln!(file, "{}", edit_warning::PURESCRIPT)?;
    module.purescript(&mut file)?;
  }

  Ok(())
}
