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
use serde_json::Value;

/// The schema of the database
#[derive(Debug)]
struct Schema {
    tables: Vec<Table>,
}

/// A table in the schema
#[derive(Debug)]
struct Table {
    /// The name of the table
    name: String,
    /// The columns in the table
    columns: Vec<Column>,
}

/// A column in the schema
#[derive(Debug)]
struct Column {
    /// The name of the column
    name: String,
    /// The type of data stored in the column
    col_type: Type,
    // todo - add support for optional columns
    // optional: bool,
}

/// Valid convex types taken from https://docs.convex.dev/database/types
///
/// Convex uses i64 and f64 for integers and floats respectively
///
/// The Null type is also an official convex type.
#[derive(Debug)]
enum Type {
    /// The table Id type
    Id(String),
    String,
    Int64,
    Float64,
    Bool,
    // Vec<u8>
    Bytes,
    // Vec<Type>
    Array,
    Object,
    Null,
}

impl Type {
    fn from_str(s: &str, table_name: Option<String>) -> Type {
        match s {
            "v.id()" => {
                if let Some(table_name) = table_name {
                    Type::Id(table_name)
                } else {
                    panic!("Table name is require for id type.")
                }
            }
            "v.int64()" => Type::Int64,
            "v.float64()" => Type::Float64,
            "v.bool()" => Type::Bool,
            "v.string()" => Type::String,
            "v.array()" => Type::Array,
            "v.bytes()" => Type::Bytes,
            "v.object()" => Type::Object,
            "v.null()" => Type::Null,
            _ => panic!("Invalid type found in schema."),
        }
    }
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

        // println!("{}", ast);

        let ast: Value = serde_json::from_str(&ast).unwrap();

        let tables = process_ast(&ast);

        println!("{:#?}", tables);

        // generate_rust_types(tables);

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

fn process_ast(ast: &Value) -> Vec<Table> {
    let mut tables = Vec::new();

    if let Some(body) = ast.get("body").and_then(|b| b.as_array()) {
        for item in body {
            if let Some(export_default) = item.get("declaration") {
                if let Some(arguments) = export_default.get("arguments").and_then(|a| a.as_array())
                {
                    for arg in arguments {
                        if let Some(properties) = arg.get("properties").and_then(|p| p.as_array()) {
                            for prop in properties {
                                if let Some(table_name) = prop
                                    .get("key")
                                    .and_then(|k| k.get("name"))
                                    .and_then(|n| n.as_str())
                                {
                                    if let Some(table_values) = prop
                                        .get("value")
                                        .and_then(|v| v.get("arguments"))
                                        .and_then(|a| a.as_array())
                                    {
                                        let mut columns = Vec::new();
                                        for table_value in table_values {
                                            if let Some(column_name) = table_value
                                                .get("key")
                                                .and_then(|k| k.get("name"))
                                                .and_then(|n| n.as_str())
                                            {
                                                if let Some(column_type_str) = table_value
                                                    .get("value")
                                                    .and_then(|v| v.get("callee"))
                                                    .and_then(|c| c.get("name"))
                                                    .and_then(|n| n.as_str())
                                                {
                                                    let column_type = Type::from_str(
                                                        column_type_str,
                                                        Some(table_name.to_string()),
                                                    );
                                                    columns.push(Column {
                                                        name: column_name.to_string(),
                                                        col_type: column_type,
                                                    });
                                                }
                                            }
                                        }
                                        tables.push(Table {
                                            name: table_name.to_string(),
                                            columns,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    tables
}

fn generate_rust_types(tables: Vec<Table>) {
    todo!()
}
