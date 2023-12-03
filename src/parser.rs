// pub mod nested;

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::ast::{ConvexFunctions, ConvexSchema, ConvexTable};
use crate::utils::{create_ast, create_debug_json, read_file_contents};

/// A parser for the AST
///
/// `ast` is the AST to parse
#[derive(Debug)]
pub(crate) struct ASTParser<'a>
{
    ast: &'a Value,
}

impl<'a> ASTParser<'a>
{
    /// Create a new ASTParser
    ///
    /// `ast` is the AST to parse
    ///
    /// Returns the created ASTParser
    pub(crate) fn new(ast: &'a Value) -> Self
    {
        Self { ast }
    }

    /// Parses the AST
    pub(crate) fn parse(&self) -> Result<ConvexSchema, String>
    {
        let mut tables = Vec::new();
        let mut functions: ConvexFunctions = HashMap::new();

        // First we need to get all the table names, we will assume all the related functions will be under the same file name
        if let Some(body) = self.ast.get("body").and_then(|b| b.as_array()) {
            for item in body {
                if let Some(export_default) = item.get("declaration") {
                    if let Some(arguments) = export_default.get("arguments").and_then(|a| a.as_array()) {
                        for arg in arguments {
                            if let Some(properties) = arg.get("properties").and_then(|p| p.as_array()) {
                                if let Some(table) = self.parse_tables(properties) {
                                    tables.extend(table);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Now we need to get all the functions from the table files.
        // We will assume that the functions are exported as default
        for table in tables.iter() {
            let file_name = format!("{}.ts", table.name);
            let file_path = format!("./convex/{}", file_name);

            let file_ast: Value = match create_ast(&file_path) {
                Ok(ast) => ast,
                Err(e) => {
                    return Err(format!("Error: {:?}", e.iter().map(|e| e.to_string()).collect::<Vec<_>>()));
                }
            };

            // create a function set for the table
            functions.insert(table.name.clone(), HashSet::new());

            // We get the function declarations from the file
            if let Some(body) = file_ast.get("body").and_then(|b| b.as_array()) {
                for b in body {
                    // because there are multiple declaration objects in the same body, we map over them and process them individually
                    // This code will run twice for each declaration in the map
                    if let Some(declaration) = b.as_object().and_then(|o| o.get("declaration")) {
                        // Inside each declaration we get the declarations array
                        if let Some(declarations) = declaration.get("declarations").and_then(|d| d.as_array()) {
                            // We iterate over the declarations and get the function name
                            for d in declarations {
                                if let Some(func_name) = d.get("id").and_then(|i| i.get("kind")).and_then(|k| k.get("name"))
                                {
                                    let name = func_name.as_str().ok_or("Function name is not a string")?.to_string();
                                    let namespace = table.name.clone();
                                    functions.get_mut(&namespace).ok_or("Namespace not found")?.insert(name);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(ConvexSchema { tables, functions })
    }

    /// Parses a table from the AST
    ///
    /// `properties` is the properties of the table
    ///
    /// Returns the parsed table or None if no table was found
    fn parse_tables(&self, properties: &[Value]) -> Option<Vec<ConvexTable>>
    {
        let mut tables = Vec::new();

        for prop in properties {
            let t_name = prop
                .get("key")
                .and_then(|k| k.get("name"))
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())?;

            let table = ConvexTable { name: t_name };

            tables.push(table);
        }

        Some(tables)
    }
}
