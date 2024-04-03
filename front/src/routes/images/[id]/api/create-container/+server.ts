import { json} from '@sveltejs/kit';
import { PUBLIC_API_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';


export const POST: RequestHandler = async ({ params, fetch }) => {
    const id = params.id;
	const serverResponse = await fetch(PUBLIC_API_URL + `/images/create-container`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            imageName: id,
            contanierName: null,
        }),
    });
    console.log('Backend response is', await serverResponse.text())
    return json('fini');
};