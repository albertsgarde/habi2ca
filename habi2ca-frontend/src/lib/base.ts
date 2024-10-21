import { onMount } from 'svelte';
import { error } from '@sveltejs/kit';
import { type Subscriber, type Unsubscriber } from 'svelte/store';
import { env } from '$env/dynamic/public';

export const BACKEND_ORIGIN: string = env.PUBLIC_BACKEND_ORIGIN || "http://localhost:8080";

export function expect<T>(value: T | null, message: string): T {
    if (value === null || value === undefined) {
        throw new Error(message);
    }
    return value;
}

export const origin = {
    subscribe(fn: Subscriber<URL | null>): Unsubscriber {
        fn(null);
        onMount(() => fn(new URL(window.location.origin)));
        return () => { };
    }
};

export async function fetchJson<T>(url: string, errorMessage: string): Promise<T> {
    return fetch(url).then(async (res) => {
        if (!res.ok) {
            console.log(res);
            const res_text = await res.text();
            if (res_text === "") {
                error(500, `${errorMessage}`);
            } else {
                error(500, `${errorMessage}: ` + await res.text());
            }
        } else {
            return res.json();
        }
    });
};
