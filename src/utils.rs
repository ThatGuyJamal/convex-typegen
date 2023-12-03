use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use oxc::allocator::Allocator;
use oxc::diagnostics::Report;
use oxc::parser::Parser;
use oxc::span::SourceType;
use serde_json::Value;

/// Read the contents of a file into a string.
///
/// `file_path` is the path to the file.
///
/// Returns a `Result` with the contents of the file as a `String`.
pub(super) fn read_file_contents(file_path: &str) -> Result<String, std::io::Error>
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Generate an AST from a file.
pub(super) fn create_ast(file: &str) -> Result<Value, Vec<Report>>
{
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(Path::new(&file)).unwrap();
    let schema_content = read_file_contents(file).unwrap();
    let ret = Parser::new(&allocator, &schema_content, source_type).parse();

    if ret.errors.is_empty() {
        let program = serde_json::to_string_pretty(&ret.program).unwrap();
        // let program: String = serde_json::to_string(&ret.program).unwrap();

        let ast: Value = serde_json::from_str(&program).unwrap();

        Ok(ast)
    } else {
        let mut reports = Vec::new();

        for error in ret.errors {
            let error = error.with_source_code(schema_content.clone());
            reports.push(error);
        }

        Err(reports)
    }
}

pub(super) fn create_debug_json(source_file: &str, content: &Value)
{
    let file_name = format!("./debug/dev/{}-{}.json", source_file, random_number());

    let mut file = File::create(&file_name).unwrap();

    let content = serde_json::to_string_pretty(content).unwrap();

    file.write_all(content.as_bytes()).unwrap();

    file.flush().unwrap();

    println!("Debug file created: {} -> {}", source_file, file_name);
}

fn random_number() -> u32
{
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = current_time.as_secs() as u32;
    let mut rng = std::num::Wrapping(seed);

    std::thread::sleep(std::time::Duration::from_secs(1));

    rng += std::num::Wrapping(1);
    

    rng.0
}

#[cfg(test)]
mod tests
{
    use std::fs::File;
    use std::io::Write;

    use tempdir::TempDir;

    use super::*;

    #[test]
    fn test_read_file_contents()
    {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new("test_dir").expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");

        // Write test content to the temporary file
        let content = "Test content for file";
        let mut file = File::create(&file_path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write content");

        // Test reading file contents
        let result = read_file_contents(file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_create_ast()
    {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new("test_dir").expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("schema.js");

        // Write test content to the temporary file
        let content = r#"
            export default defineSchema({
                users: defineTable({
                    name: v.string(),
                    post: v.id('posts'),
                }),
                post: defineTable({
                    title: v.string(),
                    body: v.string(),
                }),
            });
        "#;
        let mut file = File::create(&file_path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write content");

        // Test creating AST from a valid file
        let result = create_ast(file_path.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_random_number()
    {
        // Test random number generation
        let number1 = random_number();
        let number2 = random_number();

        // Check if generated numbers are different
        assert_ne!(number1, number2);
    }
}
