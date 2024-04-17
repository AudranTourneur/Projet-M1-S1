import { json, type RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';

export const GET: RequestHandler = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/images/');
	const res = await serverResponse.json();
	return json(res);
};
