import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import type { ComposeList } from '$lib/types/ComposeList';

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/composes/');
    const res = await serverResponse.json() as ComposeList;
    return res
};