
import type { Player } from '$lib/player';
import type { Task } from '$lib/task';
import { error } from '@sveltejs/kit';

export async function load(): Promise<{ player: Player, tasks: Task[] }> {
    const baseApiUrl = "http://localhost:8080/api";
    const playerUrl = `${baseApiUrl}/players/1`;
    const tasksUrl = `${baseApiUrl}/tasks?player=1`;
    const playerPromise = fetch(playerUrl);
    const tasksPromise = fetch(tasksUrl);

    const playerResponse = await playerPromise;
    const tasksResponse = await tasksPromise;


    if (!playerResponse.ok) {
        error(500, "Failed to fetch player data: " + playerResponse.text());
    }
    const playerJsonPromise = playerResponse.json();

    if (!tasksResponse.ok) {
        error(500, "Failed to fetch player tasks: " + tasksResponse.text());
    }
    const tasksJsonPromise = tasksResponse.json();


    return { player: await playerJsonPromise, tasks: await tasksJsonPromise };

}