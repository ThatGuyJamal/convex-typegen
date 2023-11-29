// @ts-nocheck

export default defineSchema({
	users: defineTable({
		// email: v.string(),
		// isAdmin: v.boolean(),
		// age: v.number(),
		// binary: v.bytes(),
		// customers: v.array(v.string()),
		// isNull: v.null(),
		// obj: v.object({
		// 	foo: v.string(),
		// 	bar: v.number(),
		// }),
		// post: v.id('posts'),
        // full schema cases
        optional_string: v.optional(v.string()),
        optional_obj: v.optional(v.object({
            foo: v.string(),
            bar: v.number(),
        })),
        optional_arr: v.optional(v.array(v.string())),
		arr_id: v.array(v.id('posts_arr')),
        arr_obj: v.array(v.object({
            foo: v.string(),
            bar: v.id('posts_obj'),
        })),
        obj_id: v.object({
            foo: v.string(),
            bar: v.id('posts_obj'),
        }),
        obj_obj: v.object({
            foo: v.string(),
            bar: v.object({
                foo: v.string(),
                bar: v.id('posts_obj'),
            }),
        }),
        obj_arr: v.object({
            foo: v.string(),
            bar: v.array(v.id('posts_obj')),
        }),
	}).index('by_email', ['email']),
});
