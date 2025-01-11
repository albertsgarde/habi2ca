import { BACKEND_ORIGIN, fetchJson } from '$lib/base';
import { getHabits, type Habit } from '$lib/habit';
import { getPlayers, type Player } from '$lib/player';
import { getTasks, type Task } from '$lib/task';
import type { PlayerInfo } from './playerInfo';

export async function load(): Promise<{ players: PlayerInfo[] }> {
	const players: Player[] = await getPlayers(BACKEND_ORIGIN);

	const playerPromises = players.map(async (player) => {
		const tasks: Task[] = await getTasks(BACKEND_ORIGIN, player.id);

		const habits: Habit[] = await getHabits(BACKEND_ORIGIN, player.id);

		return { player: player, numTasks: tasks.filter((task) => !task.completed).length, numHabits: habits.length };
	});
	const playerInfos = await Promise.all(playerPromises);
	return { players: playerInfos };
}
