import { json, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { z } from 'zod';
import { PUBLIC_API_URL } from '$env/static/public';

const loginRequestSchema = z.object({
	username: z.string(),
	password: z.string()
});

type LoginApiResponse =
	| {
			success: true;
			data: {
				token: string;
				expiresAt: number;
			};
	  }
	| {
			success: false;
	  };

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	const reqJson = await request.json();

	const loginRequestRes = loginRequestSchema.safeParse(reqJson);

	if (!loginRequestRes.success) {
		return text('Invalid request');
	}

	const loginRequest = loginRequestRes.data;

	const apiRes = await fetch(PUBLIC_API_URL + '', {
		method: 'POST',
		body: JSON.stringify({
			username: loginRequest.username,
			password: loginRequest.password
		})
	});

	const apiResJson = (await apiRes.json()) as LoginApiResponse;

	if (apiResJson.success == true) {
		cookies.set('token', apiResJson.data.token, {
			expires: new Date(apiResJson.data.expiresAt)
		});
		return json(apiResJson);
	} else {
		// failed to auth
		return json(apiResJson);
	}
};
