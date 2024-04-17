import { json, type RequestHandler } from '@sveltejs/kit';
import { BACKEND_API_URL } from '$lib/GlobalEnv';
import type { ContainerList } from '$lib/types/ContainerList';

export const GET: RequestHandler = async ({ fetch, params }) => {
    const { id } = params.id;
	const serverResponse = await fetch(BACKEND_API_URL + '/composes/' + id + '/stop');
	const res = (await serverResponse.json()) as ContainerList;
	return json(res);
};

