

import { BACKEND_ORIGIN, fetchJson } from '$lib/base';
import type { Player } from '$lib/player';
import type { Task } from '$lib/task';
import { error } from '@sveltejs/kit';
import type { PlayerInfo } from './playerInfo';

export async function load(): Promise<{ players: PlayerInfo[] }> {
    const players = await fetchJson(`${BACKEND_ORIGIN}/api/players`, "Failed to fetch players");

    const playerPromises = players.map(async player =>  {
        let task = await fetchJson(`${BACKEND_ORIGIN}/api/tasks?player=${player.id}`);
        return {player: player, numTasks: task.length};
    }
    );
    const playerInfos = await Promise.all(playerPromises);
    return {players: playerInfos};
}
