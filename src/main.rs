#![allow(dead_code)]

// A Code Generator for the ConvexDB Schema
// 
// The main goal of this project is to convex our schema.ts file into rust types so that
// the database can be used in a type-safe manner in rust.

use std::fs::File;
use std::io::Read;


/// Defines the schema objects in the schema.ts file.
/// 
/// This is the main entry point for the code generator.
struct SchemaObjects;

fn main() {
    let file_path = "./db/schema.ts";
    match read_file_contents(file_path) {
        Ok(contents) => {
            // `contents` contains the content of the file
            println!("File Contents:\n{}", contents);
            // Now you can process the contents of the file
            // Perform parsing or other operations here
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

