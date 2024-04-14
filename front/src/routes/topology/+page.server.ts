import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { Topology } from '$lib/types/Topology';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/topology');
	const jsonRes = (await serverResponse.json()) as Topology;
	return { ...jsonRes, metaTitle: 'Topology' };
};
