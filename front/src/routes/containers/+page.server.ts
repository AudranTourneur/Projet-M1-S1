import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { ContainerList } from '$lib/types/ContainerList';

export const load: PageServerLoad = async ({ fetch }) => {
    const serverResponse = await fetch(BACKEND_API_URL + '/containers/');
    const res = await serverResponse.json() as ContainerList;
    return { ...res, metaTitle: 'Containers list' };
};