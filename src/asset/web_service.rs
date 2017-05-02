//! Modules for generating code for serving calls over HTTP in different
//! programming environments.

/// Generate ECMAScript functions for serving calls over HTTP.
pub mod ecmascript {
  use asset::serialization::ecmascript::{deserialize, serialize};
  use source::HasSchema;
  use source::call::{ECMAScript, ECMAScriptConvention};
  use std::io;

  /// Generate an ECMAScript expression that evaluates to a function that
  /// handles a HTTP request by calling the source.
  pub fn handle<Source>(write: &mut io::Write, source: &Source)
    -> io::Result<()>
    where Source: HasSchema + ECMAScript {
    let (input_schema, output_schema) = source.schema()?;

    write!(write, "((function() {{\n")?;

    write!(write, "var deserialize = ")?;
    deserialize(write, &input_schema)?;
    write!(write, ";\n")?;

    write!(write, "var serialize = ")?;
    serialize(write, &output_schema)?;
    write!(write, ";\n")?;

    write!(write, "var call = ")?;
    source.ecmascript_call(write)?;
    write!(write, ";\n")?;

    write!(write, "return function(context, req, res) {{\n")?;
    write!(write, "var input = deserialize(req.body);\n")?;
    match source.ecmascript_convention()? {
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
