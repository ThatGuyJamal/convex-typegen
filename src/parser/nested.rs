use serde_json::Value;

use crate::ast::Type;

/// The nested AST Parser.
/// 
/// Convex supports Object and Arrays which hold other data types. This Struct is built to handle all such cases, without cluttering the main parser code.
#[derive(Debug)]
pub(super) struct Parser<'a> {
    ast: &'a Value, 
}

impl<'a> Parser<'a> {
    pub(super) fn new(ast: &'a Value) -> Self {
        Self { ast }
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
    pub(super) fn parse_optional_array_column(
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
}