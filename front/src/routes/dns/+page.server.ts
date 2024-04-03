import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import { DnsList } from '$lib/types/DnsList';

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/dns/');
    const res = await serverResponse.json() as DnsList;
    return res
};