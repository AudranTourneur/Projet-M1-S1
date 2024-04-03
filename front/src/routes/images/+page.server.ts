import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { ImageList } from '$lib/types/ImageList';

export const load: PageServerLoad = async () => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/images/');
	const res = await serverResponse.json() as ImageList;
	return res
};
