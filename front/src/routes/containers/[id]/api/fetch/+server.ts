import { json, type RequestHandler } from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';

export const GET: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(PUBLIC_API_URL + '/container/' + id);
	return json(await serverResponse.json());
};