import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { ImageData } from '$lib/types/ImageData';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(BACKEND_API_URL + '/image/' + id);
	const res = (await serverResponse.json()) as ImageData;
	return { ...res, metaTitle: 'Image ' + res.tags[0] };
};
