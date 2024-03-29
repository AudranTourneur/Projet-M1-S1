import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { ImageResponse } from '$lib/types/ImageResponse';

export const load: PageServerLoad = async () => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/images/');
	const res = await serverResponse.json() as ImageResponse;
	return res
};
