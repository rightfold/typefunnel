//! This module implements the web service source.

use Schema;
use asset::serialization::ecmascript::{deserialize, serialize};
use source::HasSchema;
use source::call::{ECMAScript, ECMAScriptConvention};
use std::io;
use std::rc::Rc;

pub struct WebService(pub Rc<Schema>, pub Rc<Schema>);

impl<'a> HasSchema for &'a WebService {
  fn schema(self) -> io::Result<(Rc<Schema>, Rc<Schema>)> {
    Ok((self.0.clone(), self.1.clone()))
  }
}

impl<'a> ECMAScript for &'a WebService {
  fn ecmascript_call(self, write: &mut io::Write) -> io::Result<()> {
    write!(write, "((function() {{\n")?;

    write!(write, "var deserialize = ")?;
    deserialize(write, &self.0)?;
    write!(write, ";\n")?;

    write!(write, "var serialize = ")?;
    serialize(write, &self.1)?;
    write!(write, ";\n")?;

    write!(write, "return function(url, input, onSuccess, onError) {{\n")?;
    write!(write, "var xhr = new XMLHttpRequest();\n")?;
    write!(write, "xhr.addEventListener('load', function() {{\n")?;
    write!(write, "var output = deserialize(JSON.parse(xhr.responseText));\n")?;
    write!(write, "onSuccess(output);\n")?;
    write!(write, "}});\n")?;
    write!(write, "xhr.open('POST', url);\n")?;
    write!(write, "xhr.send(JSON.stringify(serialize(input)));\n")?;
    write!(write, "}};\n")?;

    write!(write, "}})())")?;

    Ok(())
  }

  fn ecmascript_convention(self) -> io::Result<ECMAScriptConvention> {
    Ok(ECMAScriptConvention::Asynchronous)
  }
}
