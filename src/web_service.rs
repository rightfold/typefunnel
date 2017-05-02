//! This module implements the web service source.

use Schema;
use serialization::ecmascript::{deserialize, serialize};
use source::HasSchema;
use source::call::{ECMAScript, ECMAScriptConvention};
use std::io;
use std::rc::Rc;

pub struct WebService<'a, Source>(pub &'a Source) where Source: 'a;

impl<'a, Source> HasSchema for WebService<'a, Source> where Source: HasSchema {
  fn schema(&self) -> io::Result<(Rc<Schema>, Rc<Schema>)> {
    self.0.schema()
  }
}

impl<'a, Source> ECMAScript for WebService<'a, Source>  where Source: HasSchema {
  fn ecmascript_call(&self, write: &mut io::Write) -> io::Result<()> {
    let (input_schema, output_schema) = self.schema()?;

    write!(write, "((function() {{\n")?;

    write!(write, "var deserialize = ")?;
    deserialize(write, &input_schema)?;
    write!(write, ";\n")?;

    write!(write, "var serialize = ")?;
    serialize(write, &output_schema)?;
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

  fn ecmascript_convention(&self) -> io::Result<ECMAScriptConvention> {
    Ok(ECMAScriptConvention::Asynchronous)
  }
}

/// Generate ECMAScript functions for serving calls over HTTP.
pub mod ecmascript {
  use super::*;

  /// Generate an ECMAScript expression that evaluates to a function that
  /// handles a HTTP request by calling the source.
  pub fn handle<Source>(write: &mut io::Write, service: &WebService<Source>)
    -> io::Result<()>
    where Source: HasSchema + ECMAScript {
    let (input_schema, output_schema) = service.0.schema()?;

    write!(write, "((function() {{\n")?;

    write!(write, "var deserialize = ")?;
    deserialize(write, &input_schema)?;
    write!(write, ";\n")?;

    write!(write, "var serialize = ")?;
    serialize(write, &output_schema)?;
    write!(write, ";\n")?;

    write!(write, "var call = ")?;
    service.0.ecmascript_call(write)?;
    write!(write, ";\n")?;

    write!(write, "return function(context, req, res) {{\n")?;
    write!(write, "var input = deserialize(req.body);\n")?;
    match service.0.ecmascript_convention()? {
      ECMAScriptConvention::Synchronous => {
        write!(write, "var output = call(context, input);\n")?;
        continuation(write)?;
      },
      ECMAScriptConvention::Asynchronous => {
        write!(write, "call(context, input, function(output) {{\n")?;
        continuation(write)?;
        write!(write, "}}, function(error) {{\n")?;
        write!(write, "res.status(500);\n")?;
        write!(write, "}});\n")?;
      },
    }
    write!(write, "}};\n")?;

    write!(write, "}})())")?;

    Ok(())
  }

  fn continuation(write: &mut io::Write) -> io::Result<()> {
    write!(write, "res.json(serialize(output));\n")?;
    Ok(())
  }
}
