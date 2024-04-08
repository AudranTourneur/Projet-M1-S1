import { json, type RequestHandler } from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';

export const POST: RequestHandler = async ({ params, fetch, request }) => {
    const {ports} = await request.json();
	const { id } = params;
	const res = await fetch(`${PUBLIC_API_URL}/containers/${id}/rebind-ports`, {
		method: 'POST',
		headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            portFinal: ports,
        }),
	});
	return json(await res.json())
};

