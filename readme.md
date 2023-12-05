# Convex TypeGen

The [ConvexDB](https://www.convex.dev) Type Generator is a tool for generating the convex [schema.ts](https://docs.convex.dev/database/schemas) into rust types. I created this project
because for backend work, I love rust more than typescript, and I wanted to be able to use the convex schema in rust.

So I hope anyone using this library find it helpful. Im always open to adding more features but for now, the main goal is
to have type-checking in `query` and `mutations` on the backend. Due to complexity's, I don't have argument parsing yet for 
typescript functions, but I hope to add them in the future.

## Install

```bash
cargo install convex-typegen
```

*From https://crates.io/crates/convex-typegen*

## Edit your Cargo.toml file

After installing, you need to set the library as a build-dependency in your `Cargo.toml` file.

```toml
[build-dependencies]
convex-typegen = "0.0.1"
```

*Change to the latest version if needed*

## Create a build script

Create a `build.rs` file in your project root with the following contents:

```rust
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
```
This `build.rs` file will generate the `schema.rs` file in the `src` directory. You can change the path to whatever you want.

## Edit your main.rs file

After your schema file is generated, you need to let your `main.rs` file know about it. Add the following line to your main.rs file:

```rust
mod schema;
```

Now your all set! Your convex `query` and `mutations` will be type checked in rust!

## Example Query

```rust
client.query(schema::Users::FindAll.to_string(), maplit::btreemap! {}).await;
```

# Todo

- [ ] support function argument parsing/type checking (maybe)
