import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({fetch}) => {
	return { metaTitle: 'Settings' };
};
