// @ts-nocheck

export default defineSchema({
	users: defineTable({
		post: v.id('posts'),
	}),
	post: defineTable({
		title: v.string(),
		body: v.string(),
	}),
});
