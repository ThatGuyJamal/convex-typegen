use std::path::PathBuf;

use crate::ast::ConvexSchema;

/// Custom config options for the code generator
#[derive(Debug)]
pub struct BuilderConfig {
    /// The bath to the convex directory
    pub path: PathBuf,
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./convex"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Builder {
    pub(crate) schema: ConvexSchema,
    pub(crate) config: BuilderConfig,
}

impl Builder {
    pub(crate) fn new(schema: ConvexSchema, config: Option<BuilderConfig>) -> Self {
        if let Some(config) = config {
            Self { schema, config }
        } else {
            Self {
                schema,
                config: BuilderConfig::default(),
            }
        }
    }

    pub(crate) fn generate(&self) {
        
    }
}
