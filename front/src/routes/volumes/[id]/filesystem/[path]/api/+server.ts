import type { RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import { json } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const response = await fetch(BACKEND_API_URL + '/volume/' + params.id + '/filesystem/' + params.path);
	const serverResponseJson = await response.json();
	return json(serverResponseJson);
};
