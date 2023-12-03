//@ts-nocheck

import { query, mutation } from './_generated/server';
import { v } from 'convex/values';

export const create = mutation({
	args: {
		title: v.string(),
		body: v.string(),
		user_id: v.id('users'),
	},
	handler: async (ctx, args) => {
		let postId = await ctx.db.insert('posts', args);

		let user = await ctx.db.get(args.user_id);

		if (user) {
			let currentPost = user.post || [];
			currentPost.push(postId);
			await ctx.db.patch(user._id, { post: currentPost });
		} else {
			throw new Error('User not found');
		}

		return postId;
	},
});

export const find = query({
	args: {
		id: v.id('posts'),
	},
	handler: async (ctx, args) => {},
});
