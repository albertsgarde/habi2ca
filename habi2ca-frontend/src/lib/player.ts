import { get } from "svelte/store";

export type Player = {
    id: number;
    name: string;
    xp: number;
}

export async function addXp(origin: URL, playerId: number): Promise<Player> {
    const addXpUrl = `${origin}api/players/${playerId}/add_xp?xp=1`;
    let response = await fetch(addXpUrl, { method: 'PATCH' });
    if (response.ok) {
        return await response.json();
    } else {
        throw new Error(`Failed to add xp to player: ${await response.text()}`);
    }
}