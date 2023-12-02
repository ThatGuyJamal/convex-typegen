/// A table in the schema
#[derive(Debug)]
pub(crate) struct ConvexTable {
    /// The name of the table
    pub(crate) name: String,
}

/// A convex database function
pub(crate) struct ConvexFunction {
    /// The file the function is defined in
    pub(crate) namespace: String,
    /// The name of the function itself
    pub(crate) name: String,
}
