import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import {z} from 'zod';

const responseSchema = z.object({
    containers: z.array(
        z.object({
            id: z.string(),
            name: z.array(z.string()),
            image: z.string(),
            network: z.string(),
            networks: z.array(z.any()),
            volumes: z.array(z.string()),
            status: z.string(),
            ports: z.array(z.object({
                ip: z.string().optional(),
                privatePort: z.number(),
                publicPort: z.number().optional(),
                type: z.string()
            })),
        })
    )
});

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/containers/');
    const serverResponseJson = await serverResponse.json();
    // console.log('res', serverResponseJson)
    // const serverResponseJson = await serverResponse.json();
    // const res = responseSchema.parse(serverResponseJson);
    // console.log(res)
    return serverResponseJson
};