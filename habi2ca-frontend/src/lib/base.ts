import { onMount } from 'svelte';
import { error } from '@sveltejs/kit';
import { type Subscriber, type Unsubscriber } from 'svelte/store';
import { env } from '$env/dynamic/public';

export const BACKEND_ORIGIN: URL = new URL(env.PUBLIC_BACKEND_ORIGIN || 'http://localhost:8080');

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
		return () => {};
	}
};

export async function handleJsonResponse<T>(response: Response, errorMessage: string): Promise<T> {
	if (!response.ok) {
		console.log(response);
		const res_text = await response.text();
		if (res_text === '') {
			error(500, `${response.status}: ${errorMessage}`);
		} else {
			error(500, `${response.status}: ${errorMessage}: ` + res_text);
		}
	} else {
		return response.json();
	}
}

export async function fetchJson<T>(url: string, errorMessage: string): Promise<T> {
	return fetch(url).then(async (response) => {
		return handleJsonResponse(response, errorMessage);
	});
}
