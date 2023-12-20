import type { RequestHandler } from "@sveltejs/kit"

export const GET: RequestHandler = async ({ params, fetch, query }) => {
    return 'ok'
})
