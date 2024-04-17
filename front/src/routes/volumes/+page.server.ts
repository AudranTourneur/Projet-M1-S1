import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import { VolumeList } from '$lib/types/VolumeList';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/volumes/');
	const res = (await serverResponse.json()) as VolumeList;
	return { ...res, metaTitle: 'Volumes list' };
};