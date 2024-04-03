import { json, type RequestHandler } from "@sveltejs/kit";
import {PUBLIC_API_URL} from '$env/static/public';
import { ContainerList } from "$lib/types/ContainerList";

export const GET: RequestHandler = async ({ params, fetch}) => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/containers/');
    const res = await serverResponse.json() as ContainerList;
    return json(res)
};

