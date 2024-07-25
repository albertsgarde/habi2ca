import { onMount } from 'svelte';
import { derived, type Subscriber, type Unsubscriber } from 'svelte/store';

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

export const apiOrigin = derived(origin, $origin => {
    let api_origin = $origin;
    if (api_origin !== null) {
        api_origin.port = "8080";
    }
    return api_origin;
});