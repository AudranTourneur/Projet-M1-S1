import { PUBLIC_API_URL } from '$env/static/public';
import type { RequestHandler } from './$types';
import { error } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request }) => {
    const body = request.json();

    if (!body) {
        return error(400, 'Invalid request');
    }

    const serverResponse = await fetch(PUBLIC_API_URL + '/topology/save', {
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
    })

    return new Response(JSON.stringify({ success: true }));
};