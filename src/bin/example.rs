extern crate typefunnel;

use std::io;
use typefunnel::Schema;
use typefunnel::asset::serialization;

fn main() {
  safe_main().unwrap();
}

fn safe_main() -> io::Result<()> {
  let mut write = io::stdout();
  let schema = Schema::AllOf(vec![Schema::String, Schema::String]);
  serialization::ecmascript::serialize(&mut write, &schema)?;
  Ok(())
}
