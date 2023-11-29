// @ts-nocheck

export default defineSchema({
	users: defineTable({
		email: v.string(),
		password: v.string(),
		isAdmin: v.boolean(),
	}).index('by_email', ['email']),
});
