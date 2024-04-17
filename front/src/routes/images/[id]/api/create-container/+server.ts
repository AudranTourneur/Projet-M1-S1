import { json} from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { RequestHandler } from '@sveltejs/kit';


export const POST: RequestHandler = async ({ params, fetch , request}) => {
    const {containerName} = await request.json();
    const id = params.id;
	const serverResponse = await fetch(BACKEND_API_URL + `/images/create-container`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            imageName: id,
            containerName,
        }),
    });
    return json(await serverResponse.json());
};