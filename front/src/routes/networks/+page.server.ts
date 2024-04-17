import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { NetworkList } from '$lib/types/NetworkList';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/networks/');
	const res = (await serverResponse.json()) as NetworkList;
	return { ...res, metaTitle: 'Networks list' };
};
