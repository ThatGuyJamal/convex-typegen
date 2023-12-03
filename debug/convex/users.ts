//@ts-nocheck

import { query, mutation } from './_generated/server';
import { v } from 'convex/values';

export const create = mutation({
	args: {
		name: v.string(),
	},
	handler: async (ctx, args) => {},
});

export const find = query({
	args: {
        id: v.id('users'),
    },
	handler: async (ctx, args) => {},
});
