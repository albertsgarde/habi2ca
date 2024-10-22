import { BACKEND_ORIGIN } from '$lib/base';
import type { Player } from '$lib/player';
import type { Task } from '$lib/task';
import { error } from '@sveltejs/kit';

export async function load({
	params
}: {
	params: { playerId: string };
}): Promise<{ player: Player; tasks: Task[] }> {
	const playerIdStr = params.playerId;

	const playerId = parseInt(playerIdStr);

	if (isNaN(playerId)) {
		error(400, 'Invalid player ID');
	}

	const playerUrl = `${BACKEND_ORIGIN}/api/players/${playerId}`;
	const tasksUrl = `${BACKEND_ORIGIN}/api/tasks?player=${playerId}`;
	const playerPromise = fetch(playerUrl);
	const tasksPromise = fetch(tasksUrl);

	const playerResponse = await playerPromise;
	const tasksResponse = await tasksPromise;

	if (!playerResponse.ok) {
		error(500, 'Failed to fetch player data: ' + (await playerResponse.text()));
	}
	const playerJsonPromise = playerResponse.json();

	if (!tasksResponse.ok) {
		error(500, 'Failed to fetch player tasks: ' + (await tasksResponse.text()));
	}
	const tasksJsonPromise = tasksResponse.json();

	return { player: await playerJsonPromise, tasks: await tasksJsonPromise };
}
