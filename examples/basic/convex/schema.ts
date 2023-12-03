import { defineSchema, defineTable } from 'convex/server';
import { v } from 'convex/values';

export default defineSchema({
	users: defineTable({
		name: v.string(),
		post: v.optional(v.array(v.id('posts'))),
	}),
	post: defineTable({
		title: v.string(),
		body: v.string(),
		userId: v.id('users'),
	}),
});
