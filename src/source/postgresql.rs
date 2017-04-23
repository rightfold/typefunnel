//! This module implements the PostgreSQL query source.

use Schema;
use postgres::Connection;
use postgres::types::Type;
use source::HasSchema;
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

  /// The expected shape of the query result.
  pub shape: Shape,
}

/// The expected shape of a query result.
pub enum Shape {
  /// Any number of rows with any number of columns.
  Table,

  /// A single row with any number of columns.
  Row,

  /// A single row with a single column.
  Scalar,
}

impl<'a, 'b> HasSchema for &'a Query<'b> {
  fn schema(self) -> io::Result<Schema> {
    let statement = self.connection.prepare(&self.query)?;
    let columns = statement.columns();
    match self.shape {
      Shape::Table => unimplemented!(),
      Shape::Row => unimplemented!(),
      Shape::Scalar =>
        if columns.len() != 1 {
          Err(io::Error::new(io::ErrorKind::InvalidData, error::SCALAR))
        } else {
          type_to_schema(columns[0].type_())
        },
    }
  }
}

/// Return the schema that corresponds to a PostgreSQL type.
pub fn type_to_schema(type_: &Type) -> io::Result<Schema> {
  match *type_ {
    Type::Text => Ok(Schema::String),
    _ => unimplemented!(),
  }
}
