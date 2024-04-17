import type { RequestHandler } from "@sveltejs/kit";
import { PUBLIC_API_URL } from '$env/static/public';
import { json } from '@sveltejs/kit'

export const GET: RequestHandler = async ({ params, fetch }) => {
    console.log('params that you set as an id you dumb fuck', params);
    const { id } = params;
    const response = await fetch(PUBLIC_API_URL + '/statistics-historical/volume/' + id);
    const res = await response.json();
    return json(res);
};