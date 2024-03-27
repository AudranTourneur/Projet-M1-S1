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
            volume: z.array(z.string()),
            status: z.string(),
            ports: z.array(z.object({
                IP: z.string().optional(),
                PrivatePort: z.number(),
                PublicPort: z.number().optional(),
                Type: z.string()
            })),
        })
    )
});

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/containers/');
    const serverResponseJson = await serverResponse.json();
    console.log('res', serverResponseJson)
    // const serverResponseJson = await serverResponse.json();
    // const res = responseSchema.parse(serverResponseJson);
    // console.log(res)
    return serverResponseJson
};