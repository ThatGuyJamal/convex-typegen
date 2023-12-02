// pub mod nested;

use std::fs::File;
use std::io::Read;
use serde_json::Value;

use crate::ast::ConvexTable;

pub(super) type ConvexSchema = Vec<ConvexTable>;

/// A parser for the AST
///
/// `ast` is the AST to parse
#[derive(Debug)]
pub(crate) struct ASTParser<'a> {
    ast: &'a Value
}

impl<'a> ASTParser<'a> {
    /// Create a new ASTParser
    ///
    /// `ast` is the AST to parse
    ///
    /// Returns the created ASTParser
    pub(crate) fn new(ast: &'a Value,) -> Self {
        Self { ast }
    }

    /// Parses the AST
    pub(crate) fn parse(&self) -> ConvexSchema {
        let mut schema = Vec::new();

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
                                if let Some(table) = self.parse_tables(properties) {
                                    schema.extend(table)
                                }
                            }
                        }
                    }
                }
            }
        }

        schema
    }

    /// Parses a table from the AST
    ///
    /// `properties` is the properties of the table
    ///
    /// Returns the parsed table or None if no table was found
    fn parse_tables(&self, properties: &[Value]) -> Option<Vec<ConvexTable>> {
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

    /// Read the contents of a file into a string.
    ///
    /// `file_path` is the path to the file.
    ///
    /// Returns a `Result` with the contents of the file as a `String`.
    fn read_file_contents(&self, file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
