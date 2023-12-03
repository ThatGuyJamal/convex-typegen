// @ts-nocheck

export default defineSchema({
	users: defineTable({
		name: v.string(),
		post: v.id('posts'),
	}),
	post: defineTable({
		title: v.string(),
		body: v.string(),
	}),
});