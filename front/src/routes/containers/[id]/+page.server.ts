import type { PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { ContainerData } from '$lib/types/ContainerData';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;
	const serverResponse = await fetch(PUBLIC_API_URL + '/container/' + id);
	const serverResponseJson = (await serverResponse.json()) as ContainerData;
	return { ...serverResponseJson, metaTitle: 'Container ' + serverResponseJson.names[0].substring(1) };
};
