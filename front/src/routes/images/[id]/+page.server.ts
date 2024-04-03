import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { ImageData } from '$lib/types/ImageData';

export const load: PageServerLoad = async ({params}) => {
    const {id} = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/image/' + id);
    const res = await serverResponse.json() as ImageData;
    return res
};
