import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import type { ContainerList } from '$lib/types/ContainerList';

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/containers/');
    const res = await serverResponse.json() as ContainerList;
    return res
};