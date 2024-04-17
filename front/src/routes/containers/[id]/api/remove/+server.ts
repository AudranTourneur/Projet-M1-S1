import { json, type RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';

export const POST: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	const res = await fetch(`${BACKEND_API_URL}/containers/${id}/remove`, {
		method: 'POST'
	});
    return json(await res.json())
};

