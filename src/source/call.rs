//! Calls are procedures that conform to some schema. This module defines
//! traits for sources that can generate calls.

use Schema;
use source::HasSchema;
use std::collections::HashMap;
use std::io;

/// Trait for sources that can generate ECMAScript calls.
pub trait ECMAScript {
  /// Generate an ECMAScript expression that evaluates to a function that
  /// performs the call.
  fn ecmascript_call(&self, &mut io::Write) -> io::Result<()>;

  /// Return the calling convention used for this call.
  fn ecmascript_convention(&self) -> io::Result<ECMAScriptConvention>;
}

/// Calling convention for generated ECMAScript calls.
pub enum ECMAScriptConvention {
  /// The generated function returns the result or throws an exception.
  Synchronous,

  /// The generated function takes two callbacks.
  Asynchronous,
}

pub struct ECMAScriptModule<'a> {
  pub calls: HashMap<String, (&'a HasSchema, &'a ECMAScript)>,
}

impl<'a> ECMAScriptModule<'a> {
  pub fn ecmascript(&self, write: &mut io::Write) -> io::Result<()> {
    for (name, &(_, ref call)) in &self.calls {
      write!(write, "exports.{} = ", name)?;
      call.ecmascript_call(write)?;
      write!(write, ";\n")?;
    }
    Ok(())
  }

  pub fn purescript(&self, write: &mut io::Write) -> io::Result<()> {
    for (name, &(ref has_schema, ref call)) in &self.calls {
      let (input_schema, output_schema) = has_schema.schema()?;
      write!(write, "foreign import {} :: ", name)?;
      purescript_type(write, &input_schema)?;
      write!(write, " -> {} ", match call.ecmascript_convention()? {
        ECMAScriptConvention::Synchronous => "IOSync",
        ECMAScriptConvention::Asynchronous => "IO",
      })?;
      purescript_type(write, &output_schema)?;
      write!(write, "\n")?;
    }
    Ok(())
  }
}

fn purescript_type(write: &mut io::Write, schema: &Schema) -> io::Result<()> {
  match *schema {
    Schema::AllOf(ref elements) => {
      write!(write, "(Tuple{}", elements.len())?;
      for element in elements {
        write!(write, " ")?;
        purescript_type(write, element);
      }
      write!(write, ")")?;
      Ok(())
    },
    Schema::OneOf(_) => unimplemented!(),
    Schema::SignedInteger(_, _) => write!(write, "Int"), // FIXME: Bounds.
    Schema::SinglePrecision => unimplemented!(),
    Schema::DoublePrecision => write!(write, "Number"),
    Schema::ByteString => write!(write, "ByteString"),
    Schema::String => write!(write, "String"),
  }
}
