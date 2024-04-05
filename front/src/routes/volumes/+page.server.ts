import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { VolumeList } from '$lib/types/VolumeList';

export const load: PageServerLoad = async () => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/volumes/');
	const res = (await serverResponse.json()) as VolumeList;
	return { ...res, metaTitle: 'Volumes list' };
};
