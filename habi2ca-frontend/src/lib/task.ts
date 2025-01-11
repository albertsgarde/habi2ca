import type { Player } from './player';

export type TaskData = {
	player_id: number;
	name: string;
	description: string;
	completed: boolean;
};

export type Task = {
	id: number;
	player_id: number;
	name: string;
	description: string;
	completed: boolean;
};

export async function getTasks(origin: URL, playerId: number): Promise<Task[]> {
	const tasksUrl = `${origin}api/tasks?player=${playerId}`;
	const response = await fetch(tasksUrl);
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error(`Failed to fetch tasks: ${await response.text()}`);
	}
}

export async function createTask(origin: URL, taskData: TaskData): Promise<Task> {
	const createTaskUrl = `${origin}api/tasks`;
	const response = await fetch(createTaskUrl, {
		method: 'POST',
		body: JSON.stringify(taskData),
		headers: { 'Content-Type': 'application/json' }
	});
	if (response.ok) {
		return await response.json();
	} else {
		throw new Error(`Failed to create task: ${await response.text()}`);
	}
}

export async function completeTask(origin: URL, taskId: number): Promise<[Task, Player]> {
	const completeTaskUrl = `${origin}api/tasks/${taskId}/complete`;
	const taskResponse = await fetch(completeTaskUrl, { method: 'PATCH' });
	if (!taskResponse.ok) {
		throw new Error(`Failed to complete task: ${await taskResponse.text()}`);
	}
	const task: Task = await taskResponse.json();

	const playerResponse = await fetch(`${origin}api/players/${task.player_id}`);
	if (playerResponse.ok) {
		const player = await playerResponse.json();
		return [task, player];
	} else {
		throw new Error(`Failed to fetch player: ${await playerResponse.text()}`);
	}
}
