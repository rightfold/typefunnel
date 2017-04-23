#![warn(missing_docs)]

//! TypeFunnel is a tool for extracting schemas from sources and generating
//! assets from these schemas. Examples of sources are SQL queries, WSDL files,
//! and static data. Examples of assets are user interfaces, web services,
//! serialization, and documentation.

/// A schema describes the structure of data. Schemas are generated from
/// sources, and are used to generate web services, serialization functions,
/// user interfaces, and so on.
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
