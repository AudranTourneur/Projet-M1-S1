import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { NetworkData } from '$lib/types/NetworkData';

export const load: PageServerLoad = async ({ params }) => {
    const { id } = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/networks/' + id);
    const res = await serverResponse.json() as NetworkData;
    return res
}