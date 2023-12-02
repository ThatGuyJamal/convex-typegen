use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// Read the contents of a file into a string.
///
/// `file_path` is the path to the file.
///
/// Returns a `Result` with the contents of the file as a `String`.
pub(super) fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Generate an AST from a file.
pub(super) fn create_ast(file: &str) -> Result<Value, ()> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(Path::new(&file)).unwrap();
    let schema_content = read_file_contents(&file).unwrap();
    let ret = Parser::new(&allocator, &schema_content, source_type).parse();

    if ret.errors.is_empty() {
        let program  = serde_json::to_string_pretty(&ret.program).unwrap();
        // let program: String = serde_json::to_string(&ret.program).unwrap();

        let ast: Value = serde_json::from_str(&program).unwrap();

        Ok(ast)
    }
    else {
        for error in ret.errors {
            let error = error.with_source_code(schema_content.clone());
            println!("{error:?}");
        }

        return Err(());
    }
}


pub(super) fn create_debug_json(source_file: &str, content: &Value) {
    let file_name = format!("./debug/dev/{}-{}.json", source_file, random_number());

    let mut file = File::create(&file_name).unwrap();

    let content = serde_json::to_string_pretty(content).unwrap();

    file.write_all(content.as_bytes()).unwrap();

    file.flush().unwrap();

    println!("Debug file created: {} -> {}", source_file, file_name);
}


fn random_number() -> u32 {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = current_time.as_secs() as u32;
    let mut rng = std::num::Wrapping(seed);

    std::thread::sleep(std::time::Duration::from_secs(1));

    rng += std::num::Wrapping(1);
    let random_number = rng.0;

    random_number
}