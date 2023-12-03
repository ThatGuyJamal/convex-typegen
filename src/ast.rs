use std::collections::{HashMap, HashSet};

/// The parsed schema from schema.ts
#[derive(Debug)]
pub(super) struct ConvexSchema
{
    pub(crate) tables: Vec<ConvexTable>,
    pub(crate) functions: ConvexFunctions,
}

/// A table in the schema
#[derive(Debug)]
pub(crate) struct ConvexTable
{
    /// The name of the table
    pub(crate) name: String,
}

/// A convex database function
/// Contains the namespace of the table and the name of the convex function.
/// For example: users.ts -> create -> will be stored as key: users, value: [create]
pub(crate) type ConvexFunctions = HashMap<String, HashSet<String>>;
