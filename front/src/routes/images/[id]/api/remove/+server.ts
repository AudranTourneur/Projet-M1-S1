import type { RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';

export const POST: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	await fetch(`${BACKEND_API_URL}/images/${id}/remove`, {
		method: 'POST'
	});
	return new Response();
};
