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

## Example

```typescript
// schema.ts
import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

// Define a messages table with two indexes.
export default defineSchema({
  messages: defineTable({
    channel: v.id("channels"),
    body: v.string(),
    user: v.id("users"),
  })
    .index("by_channel", ["channel"])
    .index("by_channel_user", ["channel", "user"]),
});
```
will generate:
```rust
// models.rs
struct Messages {
    channel: String,
    body: String,
    user: i64,
}
```

This is just an example, im still in the process of developing the logic so 
this will evolve over time. However, here is the general idea.

## Todo

A list of things todo in the future for this project.

- [x] Parse typescript schema and generate AST
- [ ] Generate rust code from AST
- [ ] Add error handling to parser
- [ ] Add tests
- [ ] Make the cli file paths optional/configurable

## Contributing

If you would like to contribute to this project, please feel free to fork and
submit a pull request. I will review it as soon as I can. If you have any
questions, please feel free to reach out to me on [twitter](https://twitter.com/thatguyjamal0).