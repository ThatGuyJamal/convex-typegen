# Debug dir

I used this folder to inspect the generated AST from [schema.ts](../convex/schema.ts) and build the parser.

- [AST Simple](./ast-simple.json) This is the Simple AST output without nested objects, ids, and arrays.
- [AST Full](./ast-full.json) A complex and completed AST covering all the possible cases.

In local dev, the docs path can be found [here](file://wsl.localhost/Ubuntu/home/thatguyjamal/code/projects/convex-typegen/target/doc/convex_typegen/index.html) when using wsl2.