import type { RequestHandler } from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';

export const POST: RequestHandler = async ({ params, fetch }) => {
	const { id } = params;
	await fetch(`${PUBLIC_API_URL}/images/${id}/remove`, {
		method: 'POST'
	});
	return new Response();
};
