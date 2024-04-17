import { json, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { z } from 'zod';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { LoginResponse } from '$lib/types/LoginResponse';

const loginRequestSchema = z.object({
	username: z.string(),
	password: z.string()
});

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	try {
		const reqJson = await request.json();

		const loginRequestRes = loginRequestSchema.safeParse(reqJson);

		if (!loginRequestRes.success) {
			return text('Invalid request');
		}

		const loginRequest = loginRequestRes.data;

		const apiRes = await fetch(BACKEND_API_URL + '/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username: loginRequest.username,
				password: loginRequest.password
			})
		});

		if (apiRes.status !== 200) {
			return json({
				success: false,
				message: 'Failed to login'
			})
		}

		const apiResJson = (await apiRes.json()) as LoginResponse;

		if (apiResJson.token) {
			cookies.set('auth', apiResJson.token, {
				expires: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
				path: '/',
			});
			return json({
				success: true,
				token: apiResJson.token
			})
		}

	} catch (e) {
		console.error(e);
	}


	// failed to auth
	return json({
		success: false,
		message: 'Failed to login'
	})
};
