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
    Array(Box<Type>),
    Object,
    Null,
}

impl Type {
    fn from_str(s: &str, table_name: Option<String>, array_type: Option<Type>) -> Type {
        match s {
            "id" => {
                if let Some(table_name) = table_name {
                    Type::Id(table_name)
                } else {
                    panic!("Table name is require for id type.")
                }
            }
            "int64" => Type::Int64,
            "float64" => Type::Float64,
            "boolean" => Type::Bool,
            "string" => Type::String,
            "array" => {
                if let Some(at) = array_type {
                    Type::Array(Box::new(at))
                } else {
                    panic!("Array type is required for array type.")
                }
            }
            "bytes" => Type::Bytes,
            "object" => Type::Object,
            "null" => Type::Null,
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

/// Process the AST and extract the tables from the schema
///
/// `ast` A Json value representing the AST
///
/// Returns a vector of tables in the schema
fn process_ast(ast: &Value) -> Vec<Table> {
    // store all the tables here
    let mut tables = Vec::new();

    // the ast generates an array of json objects in the body
    // we need to iterate over each object and find the items to select out of the json
    if let Some(body) = ast.get("body").and_then(|b| b.as_array()) {
        for item in body {
            if let Some(export_default) = item.get("declaration") {
                if let Some(arguments) = export_default.get("arguments").and_then(|a| a.as_array())
                {
                    // the 'properties' array contains most of the useful information about the tables we need
                    for arg in arguments {
                        if let Some(properties) = arg.get("properties").and_then(|p| p.as_array()) {
                            // println!("Properties: {:?}", properties); // Add this line for debugging

                            for prop in properties {
                                // println!("Property: {:?}", prop); // Add this line for debugging

                                // the 'key' field contains the name of the table
                                let table_name = prop
                                    .get("key")
                                    .and_then(|k| k.get("name"))
                                    .and_then(|n| n.as_str())
                                    .unwrap()
                                    .to_string();

                                // println!("Table Name: {}", table_name); // Add this line for debugging

                                let mut columns = Vec::new();

                                if let Some(value) = prop.get("value") {
                                    // println!("Value: {:?}", value); // Add this line for debugging

                                    // get the 'callee' field object
                                    if let Some(callee) = value.get("callee").and_then(|k| {
                                        k.get("object").and_then(|k| {
                                            k.get("arguments").and_then(|p| p.as_array())
                                        })
                                    }) {
                                        // println!("Callee arguments: {:?}", callee); // Add this line for debugging

                                        // iterate over the arguments.properties of callee to get the column names and types
                                        for callee_props in callee {
                                            if let Some(arg) = callee_props
                                                .get("properties")
                                                .and_then(|p| p.as_array())
                                            {
                                                // println!("Arg: {:?}", arg); // Add this line for debugging

                                                for data in arg {
                                                    let col_name = data
                                                        .get("key")
                                                        .and_then(|k| k.get("name"))
                                                        .and_then(|n| n.as_str())
                                                        .unwrap()
                                                        .to_string();

                                                    // println!("Column Name: {}", col_name);

                                                    // the convex v type
                                                    let col_v_type = data
                                                        .get("value")
                                                        .and_then(|v| v.get("callee"))
                                                        .and_then(|v| v.get("property"))
                                                        .and_then(|p| p.get("name"))
                                                        .and_then(|n| n.as_str())
                                                        .unwrap()
                                                        .to_string();

                                                    println!("Column V Type: {}", col_v_type);

                                                    if col_v_type == "array" {
                                                        // the convex v type
                                                        if let Some(col_array_type_data) = data
                                                            .get("value")
                                                            .and_then(|v| v.get("arguments"))
                                                            .and_then(|a| a.as_array())
                                                        {
                                                            for cad in col_array_type_data {
                                                                let nested_col_type = cad
                                                                    .get("callee")
                                                                    .and_then(|c| c.get("property"))
                                                                    .and_then(|c| c.get("name"))
                                                                    .and_then(|n| n.as_str())
                                                                    .unwrap()
                                                                    .to_string();

                                                                println!(
                                                                    "Nested Column Array Type: {}",
                                                                    nested_col_type
                                                                );

                                                                let column = Column {
                                                                    name: col_name.clone(),
                                                                    col_type: {
                                                                        Type::from_str(
                                                                            &col_v_type,
                                                                            None,
                                                                            Some(
                                                                                Type::from_str(
                                                                                    &nested_col_type,
                                                                                    None,
                                                                                    None,
                                                                                ),
                                                                            ),
                                                                        )
                                                                    },
                                                                };

                                                                columns.push(column);
                                                            }
                                                        }
                                                    } else {
                                                        // create the column
                                                        let column = Column {
                                                            name: col_name,
                                                            col_type: {
                                                                Type::from_str(
                                                                    &col_v_type,
                                                                    None,
                                                                    None,
                                                                )
                                                            },
                                                        };

                                                        columns.push(column);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                let table = Table {
                                    name: table_name,
                                    columns: columns,
                                };

                                tables.push(table);
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