import { json, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';


export const POST = async ({ request, cookies }) => {
    try {
        const reqJson = await request.json();

        const apiRes = await fetch(BACKEND_API_URL + '/', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                action: "disconnect"
            })
        });

        cookies.set('auth', '', {
            expires: new Date(0),
            path: '/',
        });
        return json({
            success: true
        })


    } catch (e) {
        console.error(e);
    }

    return json({
        success: false,
        message: 'Failed to logout'
    })
}