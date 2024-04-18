import type { RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL} from '$lib/GlobalEnv';
import { json } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	const response = await fetch(BACKEND_API_URL + "/statistics-historical/volume/" + id);
	const res = await response.json();
	return json(res);
};
