use convex_typegen::generate_convex_types;

fn main()
{
    println!("cargo:rerun-if-changed=convex/schema.ts");
    
    let config = convex_typegen::Configuration {
        convex_schema_path: String::from("./convex/schema.ts"),
        code_gen_path: String::from("./src/schema.rs"),
    };

    match generate_convex_types(Some(&config)) {
        Ok(_) => {}
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }

    println!("Build.rs completed successfully!");
}
