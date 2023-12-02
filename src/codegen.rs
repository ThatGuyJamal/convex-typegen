use std::path::PathBuf;

use crate::parser::ConvexSchema;

/// Custom config options for the code generator
pub struct BuilderConfig {
    /// The bath to the convex directory
    path: PathBuf,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./convex"),
        }
    }
}

pub struct Builder<'a> {
    config: BuilderConfig,
    schema: &'a ConvexSchema,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(schema: &'a ConvexSchema, config: Option<BuilderConfig>) -> Self {
        if let Some(config) = config {
            Self { schema, config }
        } else {
            Self { schema, config: BuilderConfig::default() }
        }
    }

    pub(crate) fn generate(&self) {
        for table in self.schema {
            println!("Table: {:#?}", table);
        }
    }
}