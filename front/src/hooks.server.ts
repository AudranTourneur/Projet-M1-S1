import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	request.headers.set('cookie', event.request.headers.get('cookie') as any);
	request.headers.set('Authorization', event.cookies.get('auth') as any);

	return fetch(request);
};