export async function GET({ params, url }: { params: { api_slug: string }, url: URL }) {
    const { api_slug } = params;
    const api_url = `http://localhost:8080/api/${api_slug}?${url.searchParams.toString()}`;
    return fetch(api_url)
}

export async function POST({ params, url }: { params: { api_slug: string }, url: URL }) {
    const { api_slug } = params;
    const api_url = `http://localhost:8080/api/${api_slug}?${url.searchParams.toString()}`;
    return fetch(api_url, { method: 'POST' })
}


export async function PATCH({ params, url }: { params: { api_slug: string }, url: URL }) {
    const { api_slug } = params;
    const api_url = `http://localhost:8080/api/${api_slug}?${url.searchParams.toString()}`;
    return fetch(api_url, { method: 'PATCH' })
}


