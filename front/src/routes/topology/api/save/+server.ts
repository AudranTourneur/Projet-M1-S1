import { PUBLIC_API_URL } from '$env/static/public';
import type { RequestHandler } from './$types';
import { error } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request, fetch }) => {
    const body = await request.json();

    if (!body) {
        error(400, 'Invalid request');
    }

    try {
        const serverResponse = await fetch(PUBLIC_API_URL + '/topology/save', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(body),
        })

        return serverResponse;
    } catch (e) {
        console.log(e)
        error(500, 'Failed to save topology');
        return new Response(JSON.stringify({ success: false }));
    }
};