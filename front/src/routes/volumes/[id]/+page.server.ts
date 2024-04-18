import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { VolumeData } from '$lib/types/VolumeData';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(BACKEND_API_URL + '/volume/' + id);
	const res = (await serverResponse.json()) as VolumeData;
	return { ...res, metaTitle: 'Volume ' + res.name };
};
