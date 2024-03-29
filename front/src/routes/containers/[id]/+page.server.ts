import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { Container } from '$lib/types/Container';

export const load: PageServerLoad = async ({ params }) => {
    const { id } = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/container/' + id);
    const serverResponseJson = await serverResponse.json() as Container;
    return serverResponseJson;
};
