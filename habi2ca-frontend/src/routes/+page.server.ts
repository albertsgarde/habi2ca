
import type { Player } from '$lib/player';
import { error } from '@sveltejs/kit';

export async function load(): Promise<{ id: number, data: { name: string, xp: number } }> {
    const baseApiUrl = "http://localhost:8080/api";
    const playerUrl = `${baseApiUrl}/players/1`;
    const response = await fetch(playerUrl);


    if (response.ok) {
        const playerJson: Player = (await response.json());
        return playerJson;
    } else {
        error(500, "Failed to fetch player data: " + response.text());
    }

}