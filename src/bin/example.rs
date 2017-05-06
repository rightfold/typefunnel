extern crate typefunnel;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use typefunnel::edit_warning;
use typefunnel::source::HasSchema;
use typefunnel::source::call::{ECMAScript, ECMAScriptModule};
use typefunnel::constant::Constant;
use typefunnel::web_service;
use typefunnel::web_service::WebService;

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
  let service = WebService{
    name: "foo".to_string(),
    source: &source,
  };
  generate_server(&service)?;
  generate_client(&service)?;

  Ok(())
}

fn generate_server<Source>(service: &WebService<Source>) -> io::Result<()>
  where Source: HasSchema + ECMAScript {
  let mut file = File::create("/tmp/typefunnel/server.js")?;
  write!(file, "{}\n", edit_warning::ECMASCRIPT)?;
  web_service::ecmascript::serve(&mut file, |file| {
    web_service::ecmascript::handle(file, &service, "null")?;
    Ok(())
  })?;
  write!(file, "app.listen(1337);\n")?;
  Ok(())
}

fn generate_client<Source>(service: &WebService<Source>) -> io::Result<()>
  where Source: HasSchema + ECMAScript {
  let module = ECMAScriptModule{
    calls: {
      let mut calls = HashMap::new();
      calls.insert("remote".to_string(), (service as &HasSchema, service as &ECMAScript));
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
