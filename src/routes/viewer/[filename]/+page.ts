import { goto } from '$app/navigation';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    if (!params.filename) {
        goto('/')
    }
    return { filename: params.filename }
}

export const prerender = false
