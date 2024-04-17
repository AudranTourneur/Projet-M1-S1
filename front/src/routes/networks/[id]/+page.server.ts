import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { NetworkData } from '$lib/types/NetworkData';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(BACKEND_API_URL + '/networks/' + id);
	const res = (await serverResponse.json()) as NetworkData;
	return { ...res, metaTitle: 'Network ' + res.name };
};
