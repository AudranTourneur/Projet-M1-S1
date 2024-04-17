import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { Topology } from '$lib/types/Topology';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/topology');
	const jsonRes = (await serverResponse.json()) as Topology;
	return { ...jsonRes, metaTitle: 'Topology' };
};
