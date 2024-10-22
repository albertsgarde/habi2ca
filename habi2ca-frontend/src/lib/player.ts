export type Player = {
	id: number;
	name: string;
	xp: number;
};

export async function createPlayer(origin: URL, playerName: string): Promise<Player> {
	const createPlayerUrl = `${origin}api/players?name=${playerName}`;
	const response = await fetch(createPlayerUrl, { method: 'POST' });
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error(`Failed to create player "${playerName}": ${await response.text()}`);
	}
}

export async function addXp(origin: URL, playerId: number): Promise<Player> {
	const addXpUrl = `${origin}api/players/${playerId}/add_xp?xp=1`;
	const response = await fetch(addXpUrl, { method: 'PATCH' });
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error(`Failed to add xp to player: ${await response.text()}`);
	}
}
