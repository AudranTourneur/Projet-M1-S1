import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { z } from 'zod';

const responseSchema = z.object({
    versionDocker: z.string(),
    versionLinux: z.string(),
    images: z.number(),
    containers: z.number(),
    volumes: z.number(),
    networks: z.number(),
});

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/overview/');
    const serverResponseJson = await serverResponse.json();
    const res = responseSchema.parse(serverResponseJson);
    console.log(res)
    return res
};
