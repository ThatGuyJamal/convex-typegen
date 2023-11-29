use std::collections::HashMap;

use serde_json::Value;

use crate::ast::{Column, Table, Type};

/// The schema of the database
#[derive(Debug)]
pub(crate) struct ASTParser<'a> {
    ast: &'a Value,
}

impl<'a> ASTParser<'a> {
    pub(crate) fn new(ast: &'a Value) -> Self {
        Self { ast }
    }

    pub(crate) fn parse(&self) -> Vec<Table> {
        // store all the tables here
        let mut tables = Vec::new();

        if let Some(body) = self.ast.get("body").and_then(|b| b.as_array()) {
            for item in body {
                if let Some(export_default) = item.get("declaration") {
                    if let Some(arguments) =
                        export_default.get("arguments").and_then(|a| a.as_array())
                    {
                        // the 'properties' array contains most of the useful information about the tables we need
                        for arg in arguments {
                            // ? properties.json
                            if let Some(properties) =
                                arg.get("properties").and_then(|p| p.as_array())
                            {
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
                                                                        .and_then(|c| {
                                                                            c.get("property")
                                                                        })
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
                                                                            .and_then(|a| {
                                                                                a.as_array()
                                                                            })
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
                                                                                    p.get(
                                                                                        "property",
                                                                                    )
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
                                                                                    &value, None,
                                                                                    None, None,
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
}
