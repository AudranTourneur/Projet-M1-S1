import { env } from '$env/dynamic/private';

console.log('PORT_CORE', env.PORT_CORE);
export const BACKEND_API_URL = "http://127.0.0.1:" + (env.PORT_CORE || 8000);