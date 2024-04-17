import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { ComposeList } from '$lib/types/ComposeList';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/composes/');
	const res = (await serverResponse.json()) as ComposeList;
	return { ...res, metaTitle: 'Docker composes' };
};
