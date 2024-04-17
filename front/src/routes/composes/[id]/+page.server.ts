import type { PageServerLoad } from './$types';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import { ComposeData } from '$lib/types/ComposeData';

export const load: PageServerLoad = async ({ params, fetch }) => {
    const { id } = params;
    const serverResponse = await fetch(BACKEND_API_URL + '/composes/' + id);
    const res = await serverResponse.json() as ComposeData;
    return res
};