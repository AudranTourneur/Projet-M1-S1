import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import {z} from 'zod';

const responseSchema = z.object({
    networks: z.array(
        z.object({
            id: z.string(),
            name: z.string(),
            created: z.string(),
            labels: z.record(z.string(), z.string()),
            ipam_config: z.object({
                subnet: z.string(),
                ip_range: z.string(),
                gateway: z.string(),
                aux_addresses: z.record(z.string(), z.string())
            }),
            containers: z.object({
                name: z.string(),
                endpoint_id: z.string(),
                mac_address: z.string(),
                ipv4_address: z.string(),
                ipv6_address: z.string(),
            }),
        }),
    )
})

export const load: PageServerLoad = async () => {
    const serverResponse = await fetch(PUBLIC_API_URL + '/networks/');
    const serverResponseJson = await serverResponse.json();
    const res = responseSchema.parse(serverResponseJson);
    console.log("iii" + res)
    return res
}