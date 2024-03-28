import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import {z} from 'zod';
import  {json} from '@sveltejs/kit'

const responseSchema = z.object({
    stats: z.array(z.object({
        ts: z.number(),
        mem: z.number(),
        cpu: z.number()
    })),
}); 


export const GET: RequestHandler = async ({ params, fetch}) => {
    const {id} = params;
    const response = await fetch(PUBLIC_API_URL + '/statistics-historical/container/' + id);
    const serverResponseJson = await response.json();
    const res = responseSchema.parse(serverResponseJson);
    //console.log(res);
    return json(
        res
    );
};
