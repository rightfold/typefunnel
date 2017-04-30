//! Calls are procedures that conform to some schema. This module defines
//! traits for sources that can generate calls.

use std::io;

/// Trait for sources that can generate ECMAScript calls.
pub trait ECMAScript {
  /// Generate an ECMAScript expression that evaluates to a function that
  /// performs the call.
  fn ecmascript_call(self, &mut io::Write) -> io::Result<()>;

  /// Return the calling convention used for this call.
  fn ecmascript_convention(self) -> io::Result<ECMAScriptConvention>;
}

/// Calling convention for generated ECMAScript calls.
pub enum ECMAScriptConvention {
  /// The generated function returns the result or throws an exception.
  Synchronous,

  /// The generated function takes two callbacks.
  Asynchronous,
}
