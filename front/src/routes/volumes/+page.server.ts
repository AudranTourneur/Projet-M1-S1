import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { VolumeResponse } from '$lib/types/VolumeResponse';

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/volumes/');
    const res = await serverResponse.json() as VolumeResponse;
    return res
};
