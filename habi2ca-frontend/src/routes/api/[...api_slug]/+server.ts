export async function GET({ params, request }: { params: { api_slug: string }, request: Request }) {
    const { api_slug } = params;
    let requestUrl = new URL(request.url);
    const apiUrl = `http://localhost:8080/api/${api_slug}?${requestUrl.searchParams.toString()}`;
    let newRequest = new Request(apiUrl, { method: "GET", headers: request.headers, body: request.body });
    return fetch(newRequest)
}

export async function POST({ params, request }: { params: { api_slug: string }, request: Request }) {
    const { api_slug } = params;
    let requestUrl = new URL(request.url);
    const apiUrl = `http://localhost:8080/api/${api_slug}?${requestUrl.searchParams.toString()}`;
    let newRequest = new Request(apiUrl, { method: "POST", headers: request.headers, body: request.body, duplex: "half" });
    return fetch(newRequest)
}


export async function PATCH({ params, request }: { params: { api_slug: string }, request: Request }) {
    const { api_slug } = params;
    let requestUrl = new URL(request.url);
    const apiUrl = `http://localhost:8080/api/${api_slug}?${requestUrl.searchParams.toString()}`;
    let newRequest = new Request(apiUrl, { method: "PATCH", headers: request.headers, body: request.body });
    return fetch(newRequest)
}


