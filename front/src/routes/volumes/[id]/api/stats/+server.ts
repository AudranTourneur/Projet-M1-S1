import type { RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL} from '$lib/GlobalEnv';
import { json } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const {encoded_path} = params;
	console.log("encoded_path sent to api", encoded_path);
	const response = await fetch(BACKEND_API_URL + "/statistics-historical/volume/<encoded_path>" + encoded_path);
	const res = await response.json();
	return json(res);
};
