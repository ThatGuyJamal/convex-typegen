// @ts-nocheck

export default defineSchema({
	users: defineTable({
		email: v.string(),
		isAdmin: v.boolean(),
		age: v.number(),
		binary: v.bytes(),
		customers: v.array(v.string()),
		isNull: v.null(),
		obj: v.object({
			foo: v.string(),
			bar: v.number(),
		}),
		post: v.id('posts'),
		// todo - implement support
		// arr_post: v.array(v.id('posts_arr')),
		// obj_post: v.object({
		// 	foo: v.string(),
		// 	bar: v.id('posts_obj'),
		// }),
	}).index('by_email', ['email']),
});
