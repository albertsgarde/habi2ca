export type Level = {
	id: number;
	xp_requirement: number;
};

export async function getLevels(origin: URL): Promise<Level[]> {
	const getLevelsUrls = `${origin}api/levela`;
	const response = await fetch(getLevelsUrls, { method: 'GET' });
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error(`Failed to get levels: ${await response.text()}`);
	}
}
