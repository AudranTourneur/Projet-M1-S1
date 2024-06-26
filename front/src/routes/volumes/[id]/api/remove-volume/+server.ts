import { json } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ params, fetch }) => {
	const name = params.id;
	const response = await fetch(`${BACKEND_API_URL}/volume/${name}/remove`, {
		method: 'POST'
	});

	if (!response.ok) {
		const errorMessage = `Failed to delete volume ${name}`;
		throw new Error(errorMessage);
	}

	return json({ message: `Volume ${name} deleted.` });
};
