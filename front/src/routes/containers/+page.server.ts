import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import type { Container } from '$lib/types/Container';
import type { ContainerList } from '$lib/types/ContainerList';



export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/containers/');
    const serverResponseJson = await serverResponse.json() as ContainerList;
    // console.log('res', serverResponseJson)
    // const serverResponseJson = await serverResponse.json();
    // const res = responseSchema.parse(serverResponseJson);
    // console.log(res)
    return serverResponseJson
};