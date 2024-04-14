import { PUBLIC_API_URL } from '$env/static/public';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ route, fetch }) => {
    const currentUrl = route.id

    const res = await fetch(PUBLIC_API_URL + '/me')

    let text = await res.text()

    if (res.status !== 200 && currentUrl !== '/login') {
        throw redirect(302, '/login')
    }

    return {
        test: text,
    };
};