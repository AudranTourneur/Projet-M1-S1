import type {PageServerLoad} from './$types';
import {PUBLIC_API_URL} from '$env/static/public';
import {z} from 'zod';

const responseSchema = z.object({
        id: z.string(),
        name: z.string(),
        created: z.string(),
        labels: z.record(z.string(), z.string()),
        ipamConfig: z.array(
            z.object({
                subnet: z.string(),
                ipRange: z.string(),
                gateway: z.string(),
                auxAddresses: z.nullable(z.record(z.string(), z.string()))
            })
        ),
        containers: z.record(
            z.string(),
            z.object({
                name: z.string(),
                endpointId: z.string(),
                macAddress: z.string(),
                ipv4Address: z.string(),
                ipv6Address: z.string()
            })
        )
})

export const load: PageServerLoad = async ({params}) => {
    const {id} = params;
    const serverResponse = await fetch(PUBLIC_API_URL + '/networks/' + id);
    console.log("iiiiiiiiiiiiiiiiii")
    console.log(serverResponse)
    const serverResponseJson = await serverResponse.json();
    const res = responseSchema.parse(serverResponseJson);
    return res
}