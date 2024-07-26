import { onMount } from 'svelte';
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
