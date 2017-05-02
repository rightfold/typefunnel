//! Modules for generating code for serializing and deserializing data in
//! different programming environments.

/// Generate ECMAScript functions for serializing and deserializing data.
pub mod ecmascript {
  use Schema;
  use std::io;

  /// Generate an ECMAScript expression that evaluates to a function that
  /// serializes data conforming to the given schema.
  pub fn serialize(write: &mut io::Write, _: &Schema) -> io::Result<()> {
    write!(write, "(function(value) {{\n")?;
    write!(write, "return value;\n")?;
    write!(write, "}})")?;
    Ok(())
  }

  /// Generate an ECMAScript expression that evaluates to a function that
  /// deserializes data conforming to the given schema.
  pub fn deserialize(write: &mut io::Write, _: &Schema) -> io::Result<()> {
    write!(write, "(function(value) {{\n")?;
    write!(write, "return value;\n")?;
    write!(write, "}})")?;
    Ok(())
  }
}
