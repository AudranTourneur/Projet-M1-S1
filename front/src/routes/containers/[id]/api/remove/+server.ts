import { json, type RequestHandler } from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';

export const POST: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	const res = await fetch(`${PUBLIC_API_URL}/containers/${id}/remove`, {
		method: 'POST'
	});
    return json(await res.json())
};

