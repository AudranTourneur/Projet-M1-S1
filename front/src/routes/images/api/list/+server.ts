import { json, type RequestHandler } from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';

export const GET: RequestHandler = async ({ fetch }) => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/images/');
	const res = await serverResponse.json();
	return json(res);
};
