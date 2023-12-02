Here you will find my custom rust parser for [ConvexDB](https://www.convex.dev). The TLDr is the
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
    body: String,
    channel: ConvexId("channels"),
    user: ConvexId("users"),
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

In the [debug](./debug) folder you can see the steps I took to learn how to parse the AST json. It took hours! But I finally for it working!

## Todo

A list of things todo in the future for this project.

- [x] Parse typescript schema and generate AST
- [x] Add support for convex schema types: object, id
- [ ] Add full support for all convex schema types
- [ ] Add tests
- [ ] Generate rust code from AST
- [ ] Add error handling to parser (avoiding unwraps everywhere & passing errors to callers)
- [ ] Make the cli file paths optional/configurable

## Known Issues

- Infinite Nesting

Because of the way the data types work, it is possible to have infinite nesting within data-types. This means a user schema can look like:
```typescript
import { defineSchema, defineTable } from 'convex/server';

export default defineSchema({
	user: defineTable({
		data: v.array(v.array(v.array(v.string()))),
	}),
});
```

However, the problem with the current parser is that im manually checking inside each object and array
for the type. So if you have a schema like this, it will fail to parse as you get past the second level. I need to find a way to recursively check the types without needing to manually check each level, there should be a way to do this but im not sure how yet. If you have any ideas, please let me know! But for now, the parser will fail if you have any schema that has deep nesting.

Example:

```typescript
// schema.ts
import { defineSchema, defineTable } from 'convex/server';

export default defineSchema({
	willError: defineTable({
		data: v.array(v.array(v.array(v.string()))),
		data: v.object({
			foo: v.string(),
			bar: v.array(v.string())
		})
	}),
	willWork: defineTable({
		data: v.array(v.string())
		data: v.object({
			foo: v.string(),
			bar: v.number(),
		})
		data: v.array(v.object({
			foo: v.string(),
			bar: v.number(),
		})),
		data: v.optional(v.array(v.number())),
	})
});
```

Again, im sure theres a smarter way to do parse an AST, however im still learning rust and teaching myself about AST parsing so until I find a solution, this is the best I can do. Convex themselves to advise against deep nesting, so this should not be a problem for most people hopefully.

## Contributing

If you would like to contribute to this project, please feel free to fork and
submit a pull request. I will review it as soon as I can. If you have any
questions, please feel free to reach out to me on [twitter](https://twitter.com/thatguyjamal0).
