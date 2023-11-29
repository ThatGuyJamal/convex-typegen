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
			foo: v.number(),
			bar: v.string(),
		}),
		post: v.id('posts'),
	}).index('by_email', ['email']),
});