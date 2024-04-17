import { json, type RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(BACKEND_API_URL + '/container/' + id);
	return json(await serverResponse.json());
};