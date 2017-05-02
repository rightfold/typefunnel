#![warn(missing_docs)]

//! TypeFunnel is a tool for extracting _schemas_ and _calls_ from _sources_.
//! Examples of sources are SQL queries, WSDL files, and static data. Schemas
//! describe the structure of data. From schemas, assets can be generated.
//! Examples of such assets are user interfaces, web services, serialization,
//! and documentation. Calls are procedures that fetch or modify data
//! conforming to some schema. For example, a call generated from a SQL query
//! source will invoke the SQL query.

extern crate postgres;

pub mod constant;
pub mod edit_warning;
pub mod postgresql;
pub mod serialization;
pub mod source;
pub mod web_service;

/// A schema describes the structure of data. Schemas are generated from
/// sources, and are used to generate web services, serialization functions,
/// user interfaces, and so on.
#[derive(Debug, Eq, PartialEq)]
pub enum Schema {
  /// The data described is a heterogeneous sequence of values.
  AllOf(Vec<Schema>),

  /// The data described conforms to one of many schemas.
  OneOf(Vec<Schema>),

  /// The data described is an integer with a lower and upper bound (both
  /// inclusive).
  SignedInteger(i32, i32),

  /// The data described is a single-precision floating-point number.
  SinglePrecision,

  /// The data described is a double-precision floating-point number.
  DoublePrecision,

  /// The data described is of arbitrary format.
  ByteString,

  /// The data described is arbitrary Unicode text.
  String,
}
