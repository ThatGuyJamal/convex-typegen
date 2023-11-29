use std::collections::HashMap;

/// A table in the schema
#[derive(Debug)]
pub(crate) struct Table {
    /// The name of the table
    pub(crate) name: String,
    /// The columns in the table
    pub(crate) columns: Vec<Column>,
}

/// A column in the schema
#[derive(Debug)]
pub(crate) struct Column {
    /// The name of the column
    pub(crate) name: String,
    /// The type of data stored in the column
    pub(crate) col_type: Type,
    // todo - add support for optional columns
    // optional: bool,
}

/// Valid convex types taken from https://docs.convex.dev/database/types
///
/// Convex uses i64 and f64 for integers and floats respectively
///
/// The Null type is also an official convex type.
#[derive(Debug)]
pub(crate) enum Type {
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
    /// Convert an ast parsed string to a valid type
    /// 
    /// `s` is the string to convert
    /// 
    /// `table_name` is the name of the table the type is in. This is required for the id type.
    /// 
    /// `array_type` is the type of the array. This is required for the array type.
    /// 
    /// `object_type` is the type of the object. This is required for the object type.
    /// 
    /// Returns the matched `Type` enum
    pub(crate) fn from_str(
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
