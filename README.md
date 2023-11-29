Here you will find my custom rust parser for [ConvexDB](). The TLDr is the
convex schema is defined in a typescript file, and while convenient for development
and ease of use, when using in other type strict language its a pain to have your
database schema in two places. So I wrote a parser that will take the typescript
schema and generate rust code that can be used in your rust project.

## Installation

todo

## Usage

_This does not work yet_

```bash
convexdb-schema-parser <schema.ts> <models.rs>
```

## Example

```typescript
// schema.ts
import { defineSchema, defineTable } from 'convex/server';
import { v } from 'convex/values';

export default defineSchema({
	messages: defineTable({
		channel: v.id('channels'),
		body: v.string(),
		user: v.id('users'),
	}).index('by_channel', ['channel']),
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

Indexes don't really have any type information, so they will be ignored for now.
Index are useful inside of convex functions but not the return types themselves (to my knowledge).

So the only data we generally need is the data inside of the `defineSchema` object.

## How this works?

This is just an example, im still in the process of developing the logic so
this will evolve over time. However, here is the general idea.

The underlining logic is that we are parsing the typescript file into an AST
and then generating rust code from that AST. The AST is a tree structure that
represents the typescript file. So we can traverse the tree and generate rust
code from it. The AST is generated using the [Oxc](https://docs.rs/oxc/latest/oxc/index.html) library. This amazing library did most of the heavy lifting for me!

## Todo

A list of things todo in the future for this project.

- [x] Parse typescript schema and generate AST
- [ ] Generate rust code from AST
- [ ] Add support for all optional convex types
- [ ] Add error handling to parser
- [ ] Add tests
- [ ] Make the cli file paths optional/configurable

## Contributing

If you would like to contribute to this project, please feel free to fork and
submit a pull request. I will review it as soon as I can. If you have any
questions, please feel free to reach out to me on [twitter](https://twitter.com/thatguyjamal0).
