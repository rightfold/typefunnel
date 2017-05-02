//! This module contains the trait for sources, and implementations of included
//! sources.

use Schema;
use std::io;
use std::rc::Rc;

pub mod call;

/// Trait for sources that have schemas.
pub trait HasSchema {
  /// Return the input and output schemas of this source. May have side-effects
  /// to retrieve the schemas.
  fn schema(&self) -> io::Result<(Rc<Schema>, Rc<Schema>)>;
}
