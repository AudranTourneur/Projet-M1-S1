import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { OverviewResponse } from '$lib/types/OverviewResponse';

export const load: PageServerLoad = async ({ fetch }) => {
	const serverResponse = await fetch(PUBLIC_API_URL + '/overview/');
	const res = (await serverResponse.json()) as OverviewResponse;
	return { ...res, metaTitle: 'Overview' };
};
