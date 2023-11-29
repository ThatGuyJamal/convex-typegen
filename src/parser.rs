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
        let mut tables = Vec::new();

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

    fn parse_columns(&self, value: &Value) -> Option<Vec<Column>> {
        let mut columns = Vec::new();
        if let Some(callee) = value.get("callee").and_then(|k| {
            k.get("object")
                .and_then(|k| k.get("arguments").and_then(|p| p.as_array()))
        }) {
            for callee_props in callee {
                if let Some(arg) = callee_props.get("properties").and_then(|p| p.as_array()) {
                    for data in arg {
                        let col_name = data
                            .get("key")
                            .and_then(|k| k.get("name"))
                            .and_then(|n| n.as_str())
                            .map(|s| s.to_string())?;

                        let col_v_type = data
                            .get("value")
                            .and_then(|v| {
                                v.get("callee")
                                    .and_then(|v| v.get("property"))
                                    .and_then(|p| p.get("name"))
                                    .and_then(|n| n.as_str())
                            })
                            .map(|s| s.to_string())?;

                        // Based on col_v_type, call a function to parse columns
                        let column = match col_v_type.as_str() {
                            "array" => self.parse_array_column(col_name, col_v_type, data),
                            "object" => self.parse_object_column(col_name, col_v_type, data),
                            "id" => self.parse_id_column(col_v_type, data),
                            _ => self.parse_default_column(col_name, col_v_type, data),
                        };

                        if let Some(column) = column {
                            columns.push(column);
                        }
                    }
                }
            }
        }

        Some(columns)
    }

    fn parse_array_column(&self, c_name: String, c_type: String, data: &Value) -> Option<Column> {
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

                let column = Column {
                    name: c_name.clone(),
                    col_type: {
                        Type::from_str(
                            &c_type,
                            None,
                            Some(Type::from_str(&nested_col_type, None, None, None)),
                            None,
                        )
                    },
                };

                return Some(column);
            }
        }

        None
    }

    fn parse_object_column(&self, c_name: String, c_type: String, data: &Value) -> Option<Column> {
        let mut object_type_map: HashMap<String, Type> = HashMap::new();

        if let Some(col_object_args) = data
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

                        object_type_map.insert(key, Type::from_str(&value, None, None, None));
                    }
                }
            }

            // create the column
            let column = Column {
                name: c_name,
                col_type: { Type::from_str(&c_type, None, None, Some(object_type_map)) },
            };

            return Some(column);
        }

        None
    }

    fn parse_id_column(&self, c_type: String, data: &Value) -> Option<Column> {
        // get the name of the table that the id references
        let col_name = data
            .get("key")
            .and_then(|k| k.get("name"))
            .and_then(|n| n.as_str())
            .unwrap()
            .to_string();

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

                // create the column
                let column = Column {
                    name: col_name.clone(),
                    col_type: { Type::from_str(&c_type, Some(id_arg_name), None, None) },
                };

                return Some(column);
            }
        }
        None
    }

    fn parse_default_column(&self, c_name: String, c_type: String, data: &Value) -> Option<Column> {
        let column = Column {
            name: c_name,
            col_type: { Type::from_str(&c_type, None, None, None) },
        };

        Some(column)
    }
}
