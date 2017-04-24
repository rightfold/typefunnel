#![allow(missing_docs)]

//! Module with string constants that contain code comments to inform
//! programmers that any changes to generated code will be overwritten.

/// Macro that expands to a string literal that informs programmers that any
/// changes to generated code will be overwritten.
///
/// # Example
/// ```
/// # #[macro_use] extern crate typefunnel;
/// # fn main() {
/// pub const ECMASCRIPT: &'static str = concat!("/* ", edit_warning!(), " */");
/// # }
/// ```
#[macro_export]
macro_rules! edit_warning {
  () => ("THIS FILE WAS GENERATED USING TYPEFUNNEL! ANY CHANGES WILL BE DISCARDED!");
}

pub const C: &'static str = concat!("/* ", edit_warning!(), " */");
pub const CPLUSPLUS: &'static str = concat!("/* ", edit_warning!(), " */");
pub const CSHARP: &'static str = concat!("/* ", edit_warning!(), " */");
pub const D: &'static str = concat!("/* ", edit_warning!(), " */");
pub const ECMASCRIPT: &'static str = concat!("/* ", edit_warning!(), " */");
pub const FSHARP: &'static str = concat!("(* ", edit_warning!(), " *)");
pub const GO: &'static str = concat!("/* ", edit_warning!(), " */");
pub const HASKELL: &'static str = concat!("{- ", edit_warning!(), " -}");
pub const JAVA: &'static str = concat!("/* ", edit_warning!(), " */");
pub const OCAML: &'static str = concat!("(* ", edit_warning!(), " *)");
pub const PHP: &'static str = concat!("/* ", edit_warning!(), " */");
pub const PURESCRIPT: &'static str = concat!("{- ", edit_warning!(), " -}");
pub const RUST: &'static str = concat!("/* ", edit_warning!(), " */");
pub const SCALA: &'static str = concat!("/* ", edit_warning!(), " */");
pub const XML: &'static str = concat!("<!-- ", edit_warning!(), " -->");
