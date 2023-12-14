import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { z } from 'zod';

const responseSchema = z.object({
    volumes: z.array(
        z.object({
            name: z.string(),
            createdAt: z.string(),
            mountpoint: z.string(),
            size: z.number(),
        })
    )
});

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/volumes/');
    const serverResponseJson = await serverResponse.json();
    const res = responseSchema.parse(serverResponseJson);
    console.log(res)
    return res
};