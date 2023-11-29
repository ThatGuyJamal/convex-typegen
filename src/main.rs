#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// A Rust Type Generator for the ConvexDB Schema
//
// The main goal of this project is to convex our schema.ts file into rust types so that
// the database can be used in a type-safe manner in rust.

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;

/// The schema of the database
struct Schema {
    tables: Vec<Table>,
}

/// A table in the schema
struct Table {
    /// The name of the table
    name: String,
    /// The columns in the table
    columns: Vec<Column>,
}

/// A column in the schema
struct Column {
    /// The name of the column
    name: String,
    /// The type of data stored in the column
    col_type: Type,
    /// If the database column is optional
    optional: bool,
}

/// Valid convex types taken from https://docs.convex.dev/database/types
/// 
/// Convex uses i64 and f64 for integers and floats respectively
/// 
/// The Null type is also an official convex type.
enum Type {
    String,
    Int(i64),
    Float(f64),
    Bool(bool),
    Bytes(Vec<u8>),
    Array(Vec<Type>),
    Object(Vec<(String, Type)>),
    Null,
}

fn main() {
    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "./convex/schema.ts".to_string());

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(Path::new(&file)).unwrap();

    let schema_content = read_file_contents(&file).unwrap();

    let ret = Parser::new(&allocator, &schema_content, source_type).parse();

    if ret.errors.is_empty() {
        // let ast = serde_json::to_string_pretty(&ret.program).unwrap();
        let ast = serde_json::to_string(&ret.program).unwrap();

        println!("{}", ast);

        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(schema_content.clone());
            println!("{error:?}");
        }
    }
}

fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_ast(ast: &String) -> Vec<Table> {
    todo!()
}

fn generate_rust_types(tables: Vec<Table>) {
    todo!()
}
