import type { RequestHandler } from "@sveltejs/kit";
import { PUBLIC_API_URL } from '$env/static/public';
import { json } from '@sveltejs/kit'

export const GET: RequestHandler = async ({ params, fetch }) => {
    const { id } = params;
    const response = await fetch(PUBLIC_API_URL + '/statistics-historical/container/' + id);
    const res = await response.json();
    return json(res);
};
