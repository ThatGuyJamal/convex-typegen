#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// A Rust Type Generator for the ConvexDB Schema
//
// The main goal of this project is to convex our schema.ts file into rust types so that
// the database can be used in a type-safe manner in rust.

mod ast;
mod codegen;
mod parser;
mod utils;

use std::env;
use std::time::Instant;

use utils::create_ast;

// example user of our library
fn main() {
    generate_types().unwrap();
}

// The only function exported from the library. (+Types)
pub fn generate_types() -> std::io::Result<()> {
    let start_time = Instant::now();

    let schema_ts_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "./convex/schema.ts".to_string());

    let ast = create_ast(&schema_ts_file).unwrap();

    let schema = crate::parser::ASTParser::new(&ast).parse();

    crate::codegen::Builder::new(schema).generate("./src/schema.rs")?;

    let elapsed = start_time.elapsed();
    
    println!("Schema Type Generation Completed | Elapsed {:.2?}", elapsed);

    Ok(())
}
