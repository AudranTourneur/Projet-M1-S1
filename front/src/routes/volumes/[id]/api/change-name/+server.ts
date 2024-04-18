import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { RequestHandler } from '@sveltejs/kit';
import { json } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request, fetch }) => {
	const body = await request.json();

	const name = body.name;

	const response = await fetch(`${BACKEND_API_URL}/volume/${name}/changeName`, {
		method: 'POST'
	});

	if (!response.ok) {
		const errorMessage = `Failed to change volume name ${name}`;
		throw new Error(errorMessage);
	}

	return json({ message: `Volume ${name} changed name.` });
};
