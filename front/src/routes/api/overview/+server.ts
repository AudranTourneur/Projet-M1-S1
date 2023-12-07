import { json, text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

import { z } from 'zod';
import { PUBLIC_API_URL } from '$env/static/public';

const overviewRequestSchema = z.object({
    overview: z.string()
});