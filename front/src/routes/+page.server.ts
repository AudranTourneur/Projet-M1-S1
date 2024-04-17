import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { OverviewResponse } from '$lib/types/OverviewResponse';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(BACKEND_API_URL + '/overview/');
	const res = (await serverResponse.json()) as OverviewResponse;
	return { ...res, metaTitle: 'Overview' };
};
