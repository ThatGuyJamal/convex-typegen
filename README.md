Here you will find my custom rust parser for [ConvexDB](). The TLDr is the 
convex schema is defined in a typescript file, and while convenient for development
and ease of use, when using in other type strict language its a pain to have your 
database schema in two places. So I wrote a parser that will take the typescript
schema and generate rust code that can be used in your rust project.

## Usage

*This does not work yet*
```bash
convexdb-schema-parser <schema.ts> <models.rs>
```

## Todo

- [x] Parse typescript schema and generate AST
- [ ] Generate rust code from AST
- [ ] Add error handling to parser
- [ ] Add tests
- [ ] Make the cli file paths optional/configurable