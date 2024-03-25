import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';


export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/topology');
    const jsonRes = await serverResponse.json();
    console.log(jsonRes)
    return jsonRes;
};
