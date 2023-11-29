// @ts-nocheck

export default defineSchema({
	users: defineTable({
		email: v.string(),
		password: v.string(),
		isAdmin: v.boolean(),
		// add more fields here
		customers: v.array(v.string()),
	}).index('by_email', ['email']),
});
