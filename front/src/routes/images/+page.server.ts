import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { z } from 'zod';

const responseSchema = z.object({
	images: z.array(
		z.object({
			id: z.string(),
			tags: z.array(z.string()),
			size: z.number(),
            created: z.number(),
		})
	)
});

export const load: PageServerLoad = async () => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/images/');
	const serverResponseJson = await serverResponse.json();
	const res = responseSchema.parse(serverResponseJson);
    console.log(res)
    return res
};
