import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	request.headers.set('cookie', event.request.headers.get('cookie') as any);

    console.log('fetching', request.url, 'with cookies', request.headers.get('cookie') || 'none')

	return fetch(request);
};