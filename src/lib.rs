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

use core::panic;
use std::env;
use std::time::Instant;

use utils::create_ast;

/// The configuration for the type generator
#[derive(Debug)]
pub struct Configuration
{
    /// A path to your schema file
    pub convex_schema_path: String,
    pub code_gen_path: String,
}

/// Generate the types for the schema.ts file
///
/// `config` is an optional configuration for the type generator
///
/// # Example
/// ```rust
/// use convex_typegen::generate_types;
///
/// fn main()
/// {
///     generate_types(None);
/// }
/// ```
pub fn generate_convex_types(config: Option<&Configuration>) -> std::io::Result<()>
{
    let start_time = Instant::now();

    let schema_ts_file = match config {
        Some(config) => config.convex_schema_path.clone(),
        None => String::from("./convex/schema.ts"),
    };

    let ast = match create_ast(&schema_ts_file) {
        Ok(ast) => ast,
        Err(e) => {
            panic!("Error: {:?}", e.iter().map(|e| e.to_string()).collect::<Vec<_>>());
        }
    };

    let schema = crate::parser::ASTParser::new(&ast).parse();

    match crate::codegen::Builder::new(schema).generate("./src/schema.rs") {
        Ok(_) => {}
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }

    let elapsed = start_time.elapsed();

    println!("Schema Type Generation Completed | Elapsed {:.2?}", elapsed);

    Ok(())
}
