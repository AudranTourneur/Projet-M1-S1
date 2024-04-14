import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { NetworkList } from '$lib/types/NetworkList';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/networks/');
	const res = (await serverResponse.json()) as NetworkList;
	return { ...res, metaTitle: 'Networks list' };
};
