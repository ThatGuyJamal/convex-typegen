use std::{path::PathBuf, collections::{HashSet, HashMap}};

use crate::parser::ConvexSchema;

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
pub(crate) struct SchemeBuilderData {
    pub(crate) schema: ConvexSchema,
    /// Contains the namespace of the table and the name of the convex function.
    /// For example: users.ts -> create -> will be stored as key: users, value: [create]
    pub(crate) namespaces: HashMap<String, HashSet<String>>,
}

#[derive(Debug)]
pub(crate) struct Builder<'a> {
    pub(crate) data: &'a SchemeBuilderData,
    pub(crate) config: BuilderConfig,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(data: &'a SchemeBuilderData, config: Option<BuilderConfig>) -> Self {
        if let Some(config) = config {
            Self { data, config }
        } else {
            Self { data, config: BuilderConfig::default() }
        }
    }

    pub(crate) fn generate(&self) {
        for table in self.data.schema.iter() {
            println!("Table: {:#?}", table);
        }
    }
}