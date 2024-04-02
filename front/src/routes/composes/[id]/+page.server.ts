import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { ComposeData } from '$lib/types/ComposeData';

export const load: PageServerLoad = async ({ params }) => {
    const { id } = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/composes/' + id);
    const res = await serverResponse.json() as ComposeData;
    return res
};