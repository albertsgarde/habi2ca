import { BACKEND_ORIGIN } from '$lib/base';
import { getPlayer, type Player } from '$lib/player';
import { getTasks, type Task } from '$lib/task';
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

	const playerPromise = getPlayer(BACKEND_ORIGIN, playerId);
	const tasksPromise = getTasks(BACKEND_ORIGIN, playerId);
	return { player: await playerPromise, tasks: await tasksPromise };
}
