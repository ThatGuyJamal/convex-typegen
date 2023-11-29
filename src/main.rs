#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// A Rust Type Generator for the ConvexDB Schema
//
// The main goal of this project is to convex our schema.ts file into rust types so that
// the database can be used in a type-safe manner in rust.

use std::collections::HashMap;
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
    // f64
    Number,
    Bool,
    // Vec<u8>
    Bytes,
    // Vec<Type>
    Array(Box<Type>),
    Object(HashMap<String, Type>),
    Null,
}

impl Type {
    fn from_str(
        s: &str,
        table_name: Option<String>,
        array_type: Option<Type>,
        object_type: Option<HashMap<String, Type>>,
    ) -> Type {
        match s {
            "id" => {
                if let Some(table_name) = table_name {
                    Type::Id(table_name)
                } else {
                    panic!("Table name is require for id type.")
                }
            }
            "int64" => Type::Int64,
            "number" => Type::Number,
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
            "object" => {
                if let Some(ot) = object_type {
                    Type::Object(ot)
                } else {
                    panic!("Object type is required for object type.")
                }
            }
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
        let ast = serde_json::to_string_pretty(&ret.program).unwrap();
        // let ast = serde_json::to_string(&ret.program).unwrap();

        println!("{}", ast);

        let ast: Value = serde_json::from_str(&ast).unwrap();

        let tables = process_ast(&ast);

        // println!("{:#?}", tables);

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
                        // ? properties.json
                        if let Some(properties) = arg.get("properties").and_then(|p| p.as_array()) {
                            // ? property.txt
                            for prop in properties {
                                // the 'key' field contains the name of the table
                                let table_name = prop
                                    .get("key")
                                    .and_then(|k| k.get("name"))
                                    .and_then(|n| n.as_str())
                                    .unwrap()
                                    .to_string();

                                let mut columns = Vec::new();

                                // ? value.txt
                                if let Some(value) = prop.get("value") {
                                    // get the 'callee' field object
                                    if let Some(callee) = value.get("callee").and_then(|k| {
                                        k.get("object").and_then(|k| {
                                            k.get("arguments").and_then(|p| p.as_array())
                                        })
                                    }) {
                                        // ? callee-args.json
                                        // println!("Callee arguments: {:?}", callee); // Add this line for debugging

                                        // iterate over the arguments.properties of callee to get the column names and types
                                        for callee_props in callee {
                                            if let Some(arg) = callee_props
                                                .get("properties")
                                                .and_then(|p| p.as_array())
                                            {
                                                // println!("Arg: {:?}", arg); // Add this line for debugging

                                                for data in arg {
                                                    // the column name
                                                    let col_name = data
                                                        .get("key")
                                                        .and_then(|k| k.get("name"))
                                                        .and_then(|n| n.as_str())
                                                        .unwrap()
                                                        .to_string();

                                                    // println!("Column Name: {}", col_name);

                                                    // the column type value
                                                    let col_v_type = data
                                                        .get("value")
                                                        .and_then(|v| v.get("callee"))
                                                        .and_then(|v| v.get("property"))
                                                        .and_then(|p| p.get("name"))
                                                        .and_then(|n| n.as_str())
                                                        .unwrap()
                                                        .to_string();

                                                    // println!("Column V Type: {}", col_v_type);

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

                                                                // println!(
                                                                //     "Nested Column Array Type: {}",
                                                                //     nested_col_type
                                                                // );

                                                                // create the column
                                                                let column = Column {
                                                                    name: col_name.clone(),
                                                                    col_type: {
                                                                        Type::from_str(
                                                                            &col_v_type,
                                                                            None,
                                                                            Some(Type::from_str(
                                                                                &nested_col_type,
                                                                                None,
                                                                                None,
                                                                                None,
                                                                            )),
                                                                            None,
                                                                        )
                                                                    },
                                                                };

                                                                // add the column to the current table
                                                                columns.push(column);
                                                            }
                                                        }
                                                    } else if col_v_type == "object" {
                                                        let mut object_type_map: HashMap<
                                                            String,
                                                            Type,
                                                        > = HashMap::new();

                                                        if let Some(col_object_args) = data
                                                            .get("value")
                                                            .and_then(|v| v.get("arguments"))
                                                            .and_then(|a| a.as_array())
                                                        {
                                                            for arg_props in col_object_args {
                                                                if let Some(nested_props) =
                                                                    arg_props
                                                                        .get("properties")
                                                                        .and_then(|a| a.as_array())
                                                                {
                                                                    for arg in nested_props {
                                                                        let key = arg
                                                                            .get("key")
                                                                            .and_then(|k| {
                                                                                k.get("name")
                                                                            })
                                                                            .and_then(|n| {
                                                                                n.as_str()
                                                                            })
                                                                            .unwrap()
                                                                            .to_string();

                                                                        // println!("Key: {}", key);

                                                                        let value = arg
                                                                            .get("value")
                                                                            .and_then(|v| {
                                                                                v.get("callee")
                                                                            })
                                                                            .and_then(|p| {
                                                                                p.get("property")
                                                                            })
                                                                            .and_then(|n| {
                                                                                n.get("name")
                                                                            })
                                                                            .and_then(|n| {
                                                                                n.as_str()
                                                                            })
                                                                            .unwrap()
                                                                            .to_string();

                                                                        // println!(
                                                                        //     "Value: {}",
                                                                        //     value
                                                                        // );

                                                                        object_type_map.insert(
                                                                            key,
                                                                            Type::from_str(
                                                                                &value, None, None,
                                                                                None,
                                                                            ),
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        // create the column
                                                        let column = Column {
                                                            name: col_name,
                                                            col_type: {
                                                                Type::from_str(
                                                                    &col_v_type,
                                                                    None,
                                                                    None,
                                                                    Some(object_type_map),
                                                                )
                                                            },
                                                        };

                                                        // add the column to the current table
                                                        columns.push(column);
                                                    } else if col_v_type == "id" {
                                                        // get the name of the table that the id references
                                                        let col_name = data
                                                            .get("key")
                                                            .and_then(|k| k.get("name"))
                                                            .and_then(|n| n.as_str())
                                                            .unwrap()
                                                            .to_string();

                                                        // println!("Col Name: {}", col_name);

                                                        if let Some(id_args) = data
                                                            .get("value")
                                                            .and_then(|c| c.get("arguments"))
                                                            .and_then(|c| c.as_array())
                                                        {
                                                            for id_arg in id_args {
                                                                // get the name of the id argument
                                                                let id_arg_name = id_arg
                                                                    .get("value")
                                                                    .and_then(|n| n.as_str())
                                                                    .unwrap()
                                                                    .to_string();

                                                                // println!(
                                                                //     "Id Arg Name: {}",
                                                                //     id_arg_name
                                                                // );

                                                                // create the column
                                                                let column = Column {
                                                                    name: col_name.clone(),
                                                                    col_type: {
                                                                        Type::from_str(
                                                                            &col_v_type,
                                                                            Some(id_arg_name),
                                                                            None,
                                                                            None,
                                                                        )
                                                                    },
                                                                };

                                                                // add the column to the current table
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
                                                                    None,
                                                                )
                                                            },
                                                        };

                                                        // add the column to the current table
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
