//! Convex Type Generator
//!
//! # Features
//!
//! - Generate Rust types from the ConvexDB schema.ts file
//! - Build.rs integration
//!
//! # Example
//!
//! Create a `build.rs` file in your project root directory and add the following:
//! ```rust
//! use convex_typegen::generate_convex_types;
//!
//! fn main()
//! {
//!     println!("cargo:rerun-if-changed=convex/schema.ts");
//!
//!     let config = convex_typegen::Configuration {
//!         convex_schema_path: String::from("./convex/schema.ts"),
//!         code_gen_path: String::from("./src/schema.rs"),
//!     };
//!
//!     match generate_convex_types(Some(&config)) {
//!         Ok(_) => {}
//!         Err(e) => {
//!             panic!("Error: {:?}", e);
//!         }
//!     }
//! }
//! ```
//!
//! Then you will see a auto-generated `schema.rs` file in your `src` directory.
//!
//! You can then query the database in a type-safe manner like so:
//!
//! ```rust
//! client
//!     .query(schema::Users::FindAll.to_string(), maplit::btreemap! {})
//!     .await;
//! ```
//! You can view the examples folder in the [repository](https://github.com/ThatGuyJamal/convex-typegen/tree/master/examples) for a more detailed example.

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

/// Generate the types for the convex project.
///
/// `config` is an optional configuration for the type generator
pub fn generate_convex_types(config: Option<&Configuration>) -> std::io::Result<()>
{
    let start_time = Instant::now();

    let schema_ts_file = match config {
        Some(config) => config.convex_schema_path.clone(),
        None => String::from("./convex/schema.ts"),
    };

    let code_gen_path = match config {
        Some(config) => config.code_gen_path.clone(),
        None => String::from("./src/schema.rs"),
    };

    let ast = match create_ast(&schema_ts_file) {
        Ok(ast) => ast,
        Err(e) => {
            panic!("Error: {:?}", e.iter().map(|e| e.to_string()).collect::<Vec<_>>());
        }
    };

    let schema = match crate::parser::ASTParser::new(&ast).parse() {
        Ok(schema) => schema,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };

    match crate::codegen::Builder::new(schema).generate(&code_gen_path) {
        Ok(_) => {}
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }

    let elapsed = start_time.elapsed();

    println!("Schema Type Generation Completed | Elapsed {:.2?}", elapsed);

    Ok(())
}
