//@ts-nocheck

import { query, mutation } from './_generated/server';
import { v } from 'convex/values';

export const create = mutation({
	args: {
		name: v.string(),
	},
	handler: async (ctx, args) => {
		return await ctx.db.insert("users", {
			name: args.name,
		})
	},
});

export const find = query({
	args: {
        id: v.id('users'),
    },
	handler: async (ctx, args) => {
		return await ctx.db.get(args.id);
	},
});
