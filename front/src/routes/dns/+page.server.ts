import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { DnsList } from '$lib/types/DnsList';

export const load: PageServerLoad = async ({ fetch }) => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/dns/');
    const res = await serverResponse.json() as DnsList;
    return { ...res, metaTitle: 'DNS management' };
};