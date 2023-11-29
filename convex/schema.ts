// @ts-nocheck

export default defineSchema({
	users: defineTable({
		email: v.string(),
		isAdmin: v.boolean(),
		age: v.number(),
		binary: v.bytes(),
		customers: v.array(v.string()),
		isNull: v.null(),
		// not supported yet
		obj: v.object({
			foo: v.string(),
			bar: v.number(),
		}),
		// not supported yet
		// post: v.id('posts'),
	}).index('by_email', ['email']),
});
