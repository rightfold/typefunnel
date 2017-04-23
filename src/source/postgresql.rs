//! This module implements the PostgreSQL query source.

use Schema;
use postgres::Connection;
use postgres::stmt::{Column, Statement};
use postgres::types::Type;
use source::HasSchema;
use std::i32;
use std::io;

mod error {
  pub static SCALAR: &'static str =
    concat!("The expected shape of the query result was scalar, but the ",
            "query returns zero or more than one column.");
}

/// A SQL query source.
pub struct Query<'a> {
  /// The database connection.
  pub connection: &'a Connection,

  /// The (optionally parameterized) well-typed SQL query.
  pub query: String,

  /// The expected shape of the query parameters.
  pub input_shape: InputShape,

  /// The expected shape of the query result.
  pub output_shape: OutputShape,
}

/// The expected shape of the parameters of a query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputShape {
  /// A single row with any number of columns.
  Row,

  /// A single row with a single column.
  Scalar,
}

/// The expected shape of a query result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputShape {
  /// Any number of rows with any number of columns.
  Table,

  /// A single row with any number of columns.
  Row,

  /// A single row with a single column.
  Scalar,
}

impl<'a, 'b> HasSchema for &'a Query<'b> {
  fn schema(self) -> io::Result<(Schema, Schema)> {
    let statement = self.connection.prepare(&self.query)?;
    let input_schema = input_schema(self.input_shape, &statement)?;
    let output_schema = output_schema(self.output_shape, &statement)?;
    Ok((input_schema, output_schema))
  }
}

fn input_schema(shape: InputShape, statement: &Statement)
  -> io::Result<Schema> {
  let param_types = statement.param_types();
  match shape {
    InputShape::Row =>
      param_types.iter()
      .map(type_to_schema)
      .collect::<Result<_, _>>()
      .map(Schema::AllOf),
    InputShape::Scalar => unimplemented!(),
  }
}

fn output_schema(shape: OutputShape, statement: &Statement)
  -> io::Result<Schema> {
  let columns = statement.columns();
  match shape {
    OutputShape::Table => unimplemented!(),
    OutputShape::Row =>
      columns.iter()
      .map(Column::type_)
      .map(type_to_schema)
      .collect::<Result<_, _>>()
      .map(Schema::AllOf),
    OutputShape::Scalar =>
      if columns.len() != 1 {
        Err(io::Error::new(io::ErrorKind::InvalidData, error::SCALAR))
      } else {
        type_to_schema(columns[0].type_())
      },
  }
}

/// Return the schema that corresponds to a PostgreSQL type.
pub fn type_to_schema(type_: &Type) -> io::Result<Schema> {
  match *type_ {
    Type::Int4 => Ok(Schema::SignedInteger(i32::MIN, i32::MAX)),
    Type::Float4 => Ok(Schema::SinglePrecision),
    Type::Float8 => Ok(Schema::DoublePrecision),
    Type::Text => Ok(Schema::String),
    _ => unimplemented!(),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use postgres::TlsMode;
  use std::error::Error;
  use std::env;

  fn with_connection<F>(body: F) where F: Fn(&Connection) {
    match env::var("TYPEFUNNEL_TEST_POSTGRESQL") {
      Ok(connection_str) =>
        match Connection::connect(connection_str, TlsMode::None) {
          Ok(connection) => body(&connection),
          _ => println!("Skipping test: could not connect to PostgreSQL"),
        },
      _ =>
        println!("Skipping test: no PostgreSQL connection information given"),
    }
  }

  #[test]
  fn test_scalar_text() {
    with_connection(|connection| {
      let source = Query{
        connection: connection,
        query: "SELECT '' :: text".to_string(),
        input_shape: InputShape::Row,
        output_shape: OutputShape::Scalar,
      };
      let schema = source.schema().map_err(|e| e.description().to_string());
      assert_eq!(schema, Ok((Schema::AllOf(vec![]), Schema::String)));
    });
  }

  #[test]
  fn test_row_text() {
    with_connection(|connection| {
      let source = Query{
        connection: connection,
        query: "SELECT 0 :: int, '' :: text".to_string(),
        input_shape: InputShape::Row,
        output_shape: OutputShape::Row,
      };
      let schema = source.schema().map_err(|e| e.description().to_string());
      let expected = Schema::AllOf(vec![
        Schema::SignedInteger(i32::MIN, i32::MAX),
        Schema::String,
      ]);
      assert_eq!(schema, Ok((Schema::AllOf(vec![]), expected)));
    });
  }
}
