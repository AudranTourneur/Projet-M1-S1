import type { PageServerLoad } from "./$types";
import { PUBLIC_API_URL } from '$env/static/public';

export const load: PageServerLoad = async ({params}) => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/images/');
    const serverResponseJson = await serverResponse.json();
    return serverResponseJson;
}
