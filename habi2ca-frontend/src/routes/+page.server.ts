
import { BACKEND_ORIGIN } from '$lib/base';
import type { Player } from '$lib/player';
import type { Task } from '$lib/task';
import { error } from '@sveltejs/kit';

export async function load(): Promise<{ player: Player, tasks: Task[] }> {
    const playerUrl = `${BACKEND_ORIGIN}/api/players/1`;
    const tasksUrl = `${BACKEND_ORIGIN}/api/tasks?player=1`;
    const playerPromise = fetch(playerUrl);
    const tasksPromise = fetch(tasksUrl);

    const playerResponse = await playerPromise;
    const tasksResponse = await tasksPromise;


    if (!playerResponse.ok) {
        error(500, "Failed to fetch player data: " + await playerResponse.text());
    }
    const playerJsonPromise = playerResponse.json();

    if (!tasksResponse.ok) {
        error(500, "Failed to fetch player tasks: " + await tasksResponse.text());
    }
    const tasksJsonPromise = tasksResponse.json();


    return { player: await playerJsonPromise, tasks: await tasksJsonPromise };

}
