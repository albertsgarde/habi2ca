import { BACKEND_ORIGIN } from '$lib/base';

async function reroute(
	{ params, request }: { params: { api_slug: string }; request: Request },
	method: string
) {
	const { api_slug } = params;
	const requestUrl = new URL(request.url);
	const apiUrl = `${BACKEND_ORIGIN}/api/${api_slug}?${requestUrl.searchParams.toString()}`;
	const newRequest = new Request(apiUrl, {
		method: method,
		headers: request.headers,
		body: request.body,
		duplex: 'half'
	});
	return fetch(newRequest);
}

export async function GET({ params, request }: { params: { api_slug: string }; request: Request }) {
	return reroute({ params, request }, 'GET');
}

export async function POST({
	params,
	request
}: {
	params: { api_slug: string };
	request: Request;
}) {
	return reroute({ params, request }, 'POST');
}

export async function PATCH({
	params,
	request
}: {
	params: { api_slug: string };
	request: Request;
}) {
	return reroute({ params, request }, 'PATCH');
}
