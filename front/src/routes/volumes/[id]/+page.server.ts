import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { VolumeData } from '$lib/types/VolumeData';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(PUBLIC_API_URL + '/volume/' + id);
	const res = (await serverResponse.json()) as VolumeData;
	return { ...res, metaTitle: 'Volume ' + res.name };
};


