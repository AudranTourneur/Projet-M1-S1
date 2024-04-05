import type { RequestHandler } from "@sveltejs/kit";
import {PUBLIC_API_URL} from '$env/static/public';
import  {json} from '@sveltejs/kit'

export const GET: RequestHandler = async ({params}) => {
    const response = await fetch(PUBLIC_API_URL + '/volume/' + params.id + '/filesystem/' + params.path);
    const serverResponseJson = await response.json();
    return json(serverResponseJson);
};

