//! This module implements a source that exports constant data.

use Schema;
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
