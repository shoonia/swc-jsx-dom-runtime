use std::{env, process};

use wasm_opt::{Feature, OptimizationOptions};

fn main() {
  let arguments: Vec<_> = env::args_os().collect();
  if arguments.len() != 3 {
    eprintln!("usage: optimize-wasm INPUT OUTPUT");
    process::exit(2);
  }

  if let Err(error) = OptimizationOptions::new_optimize_for_size_aggressively()
    .enable_feature(Feature::BulkMemory)
    .run(&arguments[1], &arguments[2])
  {
    eprintln!("wasm optimization failed: {error}");
    process::exit(1);
  }
}
