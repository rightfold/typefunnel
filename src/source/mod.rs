//! This module contains the trait for sources, and implementations of included
//! sources.

use Schema;
use std::io;

pub mod postgresql;

/// Trait for sources that have schemas.
pub trait HasSchema {
  /// Return the schema of this source. May have side-effects to retrieve the
  /// schema.
  fn schema(self) -> io::Result<Schema>;
}
