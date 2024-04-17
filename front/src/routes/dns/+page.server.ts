import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { DnsList } from '$lib/types/DnsList';

export const load: PageServerLoad = async ({ fetch }) => {
    const serverResponse = await fetch(BACKEND_API_URL + '/dns/');
    const res = await serverResponse.json() as DnsList;
    return { ...res, metaTitle: 'DNS management' };
};