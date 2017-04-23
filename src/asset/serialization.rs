//! Modules for generating code for serializing and deserializing data in
//! different programming environments.

/// Generate ECMAScript functions for serializing and deserializing data.
pub mod ecmascript {
  use Schema;
  use std::io;

  /// Generate an ECMAScript expression that evaluates to a function that
  /// serializes data conforming to the given schema.
  pub fn serialize(write: &mut io::Write, schema: &Schema) -> io::Result<()> {
    write!(write, "(function(document, namespace, parent, value) {{\n")?;
    write!(write, "var node;\n")?;
    match *schema {
      Schema::AllOf(ref element_schemas) => {
        for (index, element_schema) in element_schemas.iter().enumerate() {
          write!(write, "node = document.createElementNS(namespace, 'element-{}');\n", index)?;
          write!(write, "parent.appendChild(node);\n")?;
          serialize(write, element_schema)?;
          write!(write, "(document, namespace, node, value[{}]);\n", index)?;
        }
      },
      Schema::OneOf(_) =>
        unimplemented!(),
      Schema::SignedInteger(_, _) |
      Schema::SinglePrecision |
      Schema::DoublePrecision => {
        write!(write, "node = document.createTextNode('' + value);\n")?;
        write!(write, "parent.appendChild(node);\n")?;
      },
      Schema::ByteString =>
        unimplemented!(),
      Schema::String => {
        write!(write, "node = document.createTextNode(value);\n")?;
        write!(write, "parent.appendChild(node);\n")?;
      },
    }
    write!(write, "}})")?;
    Ok(())
  }
}
