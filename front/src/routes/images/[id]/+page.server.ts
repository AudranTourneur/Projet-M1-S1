import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { z } from 'zod';

const responseSchema = z.object({
    id: z.string(),
    tags: z.array(z.string()),
    size: z.number(),
    created: z.number(),
    history: z.array(z.object({
        id: z.string(),
        created: z.number(),
        createdBy: z.string(),
        tags: z.array(z.string()),
        size: z.number(),
        comment: z.string()
    }))
});

export const load: PageServerLoad = async ({params}) => {
    const {id} = params;
    console.log('======================================== getting', id)
    const serverResponse = await fetch(PUBLIC_API_URL + '/image/' + id);
    const serverResponseJson = await serverResponse.json();
    const res = responseSchema.parse(serverResponseJson);
    console.log(res)
    return res
};
