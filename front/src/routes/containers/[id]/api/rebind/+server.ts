import { json, type RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';

export const POST: RequestHandler = async ({ params, fetch, request }) => {
    const   ports = await request.json();
    const { id } = params;
    const res = await fetch(`${BACKEND_API_URL}/containers/${id}/rebind-ports`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(ports),
    });
    return json(await res.json())
};

