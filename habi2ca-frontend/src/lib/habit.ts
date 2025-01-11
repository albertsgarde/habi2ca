import type { Player } from './player';

export type HabitData = {
	player_id: number;
	name: string;
	description: string;
};

export type Habit = {
	id: number;
	player_id: number;
	name: string;
	description: string;
};

export async function getHabits(origin: URL, playerId: number): Promise<Habit[]> {
	const habitsUrl = `${origin}api/habits?player=${playerId}`;
	const response = await fetch(habitsUrl);
	if (response.ok) {
		return await response.json();
	} else {
		const statusCode = response.status;
		throw new Error(`Failed to fetch habits. ${statusCode}: ${await response.text()}`);
	}
}

export async function createHabit(origin: URL, habitData: HabitData): Promise<Habit> {
	const createHabitUrl = `${origin}api/habits`;
	const response = await fetch(createHabitUrl, {
		method: 'POST',
		body: JSON.stringify(habitData),
		headers: { 'Content-Type': 'application/json' }
	});
	if (response.ok) {
		return await response.json();
	} else {
		const statusCode = response.status;
		throw new Error(`Failed to create habit: ${await response.text()}`);
	}
}

export async function incrementHabit(origin: URL, habitId: number): Promise<[Habit, Player]> {
	const completeHabitUrl = `${origin}api/habits/${habitId}/increment`;
	const habitResponse = await fetch(completeHabitUrl, { method: 'PATCH' });
	if (!habitResponse.ok) {
		throw new Error(`Failed to increment habit. ${habitResponse.status}: ${await habitResponse.text()}`);
	}
	const habit: Habit = await habitResponse.json();

	const playerResponse = await fetch(`${origin}api/players/${habit.player_id}`);
	if (playerResponse.ok) {
		const player = await playerResponse.json();
		return [habit, player];
	} else {
		const statusCode = playerResponse.status;
		throw new Error(`Failed to fetch player. ${statusCode}: ${await playerResponse.text()}`);
	}
}