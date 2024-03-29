import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import { z } from 'zod';
import type { OurNetwork } from '$lib/types/OurNetwork';

export const load: PageServerLoad = async ({ params }) => {
    const { id } = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/networks/' + id);
    const serverResponseJson = await serverResponse.json() as OurNetwork;
    return res
}