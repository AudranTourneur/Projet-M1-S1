import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { NetworkResponse } from '$lib/types/NetworkResponse';

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/networks/');
    const res = await serverResponse.json() as NetworkResponse;
    return res
}