import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { ImageList } from '$lib/types/ImageList';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/images/');
	const res = await serverResponse.json() as ImageList;
	return { ...res, metaTitle: 'Images list' };
};
