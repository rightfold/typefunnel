//! This module implements a source that exports constant data.

use Schema;
use source::call::{ECMAScript, ECMAScriptConvention};
use source::HasSchema;
use std::io;

/// Constant data.
#[allow(missing_docs)]
pub enum Constant {
  AllOf(Vec<Constant>),
  SignedInteger(i32),
  SinglePrecision(f32),
  DoublePrecision(f64),
  ByteString(Vec<u8>),
  String(String),
}

impl<'a> HasSchema for &'a Constant {
  fn schema(self) -> io::Result<(Schema, Schema)> {
    Ok((Schema::AllOf(vec![]), output_schema(self)))
  }
}

impl<'a> ECMAScript for &'a Constant {
  fn ecmascript_call(self, write: &mut io::Write) -> io::Result<()> {
    write!(write, "(function() {{\nreturn ")?;
    ecmascript_expression(write, self)?;
    write!(write, ";\n}})")?;
    Ok(())
  }

  fn ecmascript_convention(self) -> io::Result<ECMAScriptConvention> {
    Ok(ECMAScriptConvention::Synchronous)
  }
}

fn output_schema(constant: &Constant) -> Schema {
  match *constant {
    Constant::AllOf(ref elements) =>
      Schema::AllOf(
        elements.iter()
        .map(output_schema)
        .collect(),
      ),
    Constant::SignedInteger(value) => Schema::SignedInteger(value, value),
    Constant::SinglePrecision(_) => Schema::SinglePrecision,
    Constant::DoublePrecision(_) => Schema::DoublePrecision,
    Constant::ByteString(_) => Schema::ByteString,
    Constant::String(_) => Schema::String,
  }
}

fn ecmascript_expression(write: &mut io::Write, constant: &Constant)
  -> io::Result<()> {
  match *constant {
    Constant::AllOf(ref elements) => {
      write!(write, "[\n")?;
      for element in elements {
        ecmascript_expression(write, element)?;
        write!(write, ",\n")?;
      }
      write!(write, "]")?;
      Ok(())
    },
    Constant::SignedInteger(value) => write!(write, "{}", value),
    Constant::SinglePrecision(value) => write!(write, "{}", value),
    Constant::DoublePrecision(value) => write!(write, "{}", value),
    Constant::ByteString(_) => unimplemented!(),
    Constant::String(ref value) => write!(write, "'{}'", value),
  }
}
