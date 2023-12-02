use std::collections::HashMap;

use serde_json::Value;

use crate::ast::{Column, Table, Type};

/// A parser for the AST
///
/// `ast` is the AST to parse
#[derive(Debug)]
pub(crate) struct ASTParser<'a> {
    ast: &'a Value,
}

impl<'a> ASTParser<'a> {
    /// Create a new ASTParser
    ///
    /// `ast` is the AST to parse
    ///
    /// Returns the created ASTParser
    pub(crate) fn new(ast: &'a Value) -> Self {
        Self { ast }
    }

    /// Parses the AST
    pub(crate) fn parse(&self) -> Vec<Table> {
        let mut tables = Vec::new();

        // Most of the important data in the ast is under all these json objects so we simply parse them and loop through
        // there properties to get the data we need.
        if let Some(body) = self.ast.get("body").and_then(|b| b.as_array()) {
            for item in body {
                if let Some(export_default) = item.get("declaration") {
                    if let Some(arguments) =
                        export_default.get("arguments").and_then(|a| a.as_array())
                    {
                        for arg in arguments {
                            if let Some(properties) =
                                arg.get("properties").and_then(|p| p.as_array())
                            {
                                if let Some(table) = self.parse_table(properties) {
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

    /// Parses a table from the AST
    ///
    /// `properties` is the properties of the table
    ///
    /// Returns the parsed table or None if no table was found
    fn parse_table(&self, properties: &[Value]) -> Option<Table> {
        for prop in properties {
            let table_name = prop
                .get("key")
                .and_then(|k| k.get("name"))
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())?;

            let columns = if let Some(value) = prop.get("value") {
                self.parse_columns(value)?
            } else {
                vec![]
            };

            return Some(Table {
                name: table_name,
                columns,
            });
        }
        None
    }

    /// Parses the columns of a table from the AST
    ///
    /// `value` is the value of the table
    ///
    /// Returns the parsed columns or None if no columns were found
    fn parse_columns(&self, value: &Value) -> Option<Vec<Column>> {
        let mut columns = Vec::new();

        if let Some(callee) = value.get("callee").and_then(|k| {
            k.get("object")
                .and_then(|k| k.get("arguments").and_then(|p| p.as_array()))
        }) {
            for callee_props in callee {
                if let Some(arg) = callee_props.get("properties").and_then(|p| p.as_array()) {
                    for ast in arg {
                        let col_name = ast
                            .get("key")
                            .and_then(|k| k.get("name"))
                            .and_then(|n| n.as_str())
                            .map(|s| s.to_string())?;

                        let col_v_type = ast
                            .get("value")
                            .and_then(|v| {
                                v.get("callee")
                                    .and_then(|v| v.get("property"))
                                    .and_then(|p| p.get("name"))
                                    .and_then(|n| n.as_str())
                            })
                            .map(|s| s.to_string())?;

                        // Based on col_v_type, call a function to parse columns
                        let current_column_token =
                            self.parse_column_data_types(col_name, col_v_type, ast);

                        if let Some(column) = current_column_token {
                            columns.push(column);
                        } else {
                            continue;
                        }
                    }
                }
            }
        }

        Some(columns)
    }

    /// Parse the current column data types and get there ast data depending on what values they contain
    ///
    /// `col_type` is the type of the column
    ///
    /// `name` is the name of the column
    ///
    /// `current_ast` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_column_data_types(
        &self,
        col_name: String,
        col_type: String,
        current_ast: &Value,
    ) -> Option<Column> {
        match col_type.as_str() {
            "array" => self.parse_array_column(col_name, col_type, current_ast),
            "object" => self.parse_object_column(col_name, col_type, current_ast),
            "id" => self.parse_id_column(col_type, current_ast),
            "optional" => self.parse_optional_column(col_name, col_type, current_ast),
            _ => self.parse_default_column(col_name, col_type, current_ast),
        }
    }

    /// Parses an array column from the AST
    ///
    /// `c_name` is the name of the column
    ///
    /// `c_type` is the type of the column
    ///
    /// `data` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_array_column(&self, c_name: String, c_type: String, ast: &Value) -> Option<Column> {
        if let Some(col_array_type_data) = ast
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

                let column = Column {
                    name: c_name.clone(),
                    col_type: {
                        Type::from_str(
                            &c_type,
                            None,
                            Some(Type::from_str(&nested_col_type, None, None, None, None)),
                            None,
                            None,
                        )
                    },
                };

                return Some(column);
            }
        }

        None
    }

    /// Parses an object column from the AST
    ///
    /// `c_name` is the name of the column
    ///
    /// `c_type` is the type of the column
    ///
    /// `data` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_object_column(&self, c_name: String, c_type: String, ast: &Value) -> Option<Column> {
        let mut object_type_map: HashMap<String, Type> = HashMap::new();

        if let Some(col_object_args) = ast
            .get("value")
            .and_then(|v| v.get("arguments"))
            .and_then(|a| a.as_array())
        {
            for arg_props in col_object_args {
                if let Some(nested_props) = arg_props.get("properties").and_then(|a| a.as_array()) {
                    for arg in nested_props {
                        let key = arg
                            .get("key")
                            .and_then(|k| k.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap()
                            .to_string();

                        // println!("Key: {}", key);

                        let value = arg
                            .get("value")
                            .and_then(|v| v.get("callee"))
                            .and_then(|p| p.get("property"))
                            .and_then(|n| n.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap()
                            .to_string();

                        // println!(
                        //     "Value: {}",
                        //     value
                        // );

                        object_type_map.insert(key, Type::from_str(&value, None, None, None, None));
                    }
                }
            }

            // create the column
            let column = Column {
                name: c_name,
                col_type: { Type::from_str(&c_type, None, None, Some(object_type_map), None) },
            };

            return Some(column);
        }

        None
    }

    /// Parses an id column from the AST
    ///
    /// `c_type` is the type of the column
    ///
    /// `data` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_id_column(&self, c_type: String, ast: &Value) -> Option<Column> {
        // get the name of the table that the id references
        let col_name = ast
            .get("key")
            .and_then(|k| k.get("name"))
            .and_then(|n| n.as_str())
            .unwrap()
            .to_string();

        if let Some(id_args) = ast
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

                // create the column
                let column = Column {
                    name: col_name.clone(),
                    col_type: { Type::from_str(&c_type, Some(id_arg_name), None, None, None) },
                };

                return Some(column);
            }
        }
        None
    }

    /// Parses an optional column from the AST
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_optional_column(&self, c_name: String, c_type: String, ast: &Value) -> Option<Column> {
        println!("Column type: {}", c_type);
        // println!("Column ast: {}", serde_json::to_string_pretty(ast).unwrap());

        // Because optional types can store any type of data, we need to get the type of data that the optional type stores, and then
        // do extra parsing depending on what type of data it is.
        if let Some(args) = ast
            .get("value")
            .and_then(|v| v.get("arguments"))
            .and_then(|a| a.as_array())
        {
            for arg in args {
                let arg_type = arg
                    .get("callee")
                    .and_then(|p| p.get("property"))
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap()
                    .to_string();

                // Get the valid nested values.
                // its important to pass the `ast` and not arg ast as we need each column to have the same base ast data to work with.
                if arg_type == "array" {
                    let col_metadata =
                        self.parse_optional_array_column(c_name.clone(), arg_type, ast);

                    let column = Column {
                        name: c_name.clone(),
                        col_type: { Type::from_str(&c_type, None, None, None, col_metadata) },
                    };

                    return Some(column);
                }

                if arg_type == "object" {
                    // self.parse_optional_object_column(c_name, c_type, arg)
                    todo!("Optional object column parsing")
                }

                let column = Column {
                    name: c_name.clone(),
                    col_type: {
                        Type::from_str(
                            &c_type,
                            None,
                            None,
                            None,
                            Some(Type::from_str(&arg_type, None, None, None, None)),
                        )
                    },
                };

                return Some(column);
            }
        }

        None
    }

    /// Parses an optional array column from the AST
    ///
    /// `c_name` is the name of the column
    ///
    /// `c_type` is the type of the column
    ///
    /// `ast` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_optional_array_column(
        &self,
        c_name: String,
        c_type: String,
        ast: &Value,
    ) -> Option<Type> {
        // println!("Optional array column ast: {}", serde_json::to_string_pretty(ast).unwrap());
        // println!("Optional array column type: {}", c_type);
        if let Some(args) = ast
            .get("value")
            .and_then(|v| v.get("arguments"))
            .and_then(|a| a.as_array())
        {
            for arg in args {
                if let Some(arg_props) = arg.get("arguments").and_then(|a| a.as_array()) {
                    let nested_col_type = arg_props
                        .get(0)
                        .and_then(|c| c.get("callee"))
                        .and_then(|c| c.get("property"))
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap()
                        .to_string();

                    let type_data = Type::from_str(
                        "array",
                        None,
                        Some(Type::from_str(&nested_col_type, None, None, None, None)),
                        None,
                        None,
                    );

                    return Some(type_data);
                }
            }
        }

        None
    }

    /// Parses all other columns from the AST
    ///
    /// These are basic types that don't have deep nesting (strings, numbers, booleans, etc)
    ///
    /// `c_name` is the name of the column
    ///
    /// `c_type` is the type of the column
    ///
    /// `data` is the ast data of the column
    ///
    /// Returns the parsed column or None if no column was found
    fn parse_default_column(&self, c_name: String, c_type: String, data: &Value) -> Option<Column> {
        let column = Column {
            name: c_name,
            col_type: { Type::from_str(&c_type, None, None, None, None) },
        };

        Some(column)
    }
}
