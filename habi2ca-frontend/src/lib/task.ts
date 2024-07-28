import type { Player } from "./player";

export type TaskData = {
    player: number;
    name: string;
    description: string;
    completed: boolean;
}

export type Task = {
    id: number;
    player: number;
    name: string;
    description: string;
    completed: boolean;
}

export async function getTasks(origin: URL, playerId: number): Promise<Task[]> {
    const tasksUrl = `${origin}api/tasks?player=${playerId}`;
    let response = await fetch(tasksUrl);
    if (response.ok) {
        return await response.json();
    } else {
        throw new Error(`Failed to fetch tasks: ${await response.text()}`);
    }
}

export async function createTask(origin: URL, taskData: TaskData): Promise<Task> {
    const createTaskUrl = `${origin}api/tasks`;
    let response = await fetch(createTaskUrl, { method: 'POST', body: JSON.stringify(taskData), headers: { "Content-Type": "application/json" } });
    if (response.ok) {
        return await response.json();
    } else {
        throw new Error(`Failed to create task: ${await response.text()}`);
    }
}

export async function completeTask(origin: URL, taskId: number): Promise<[Task, Player]> {
    const completeTaskUrl = `${origin}api/tasks/${taskId}/complete`;
    let taskResponse = await fetch(completeTaskUrl, { method: 'PATCH' });
    if (!taskResponse.ok) {
        throw new Error(`Failed to complete task: ${await taskResponse.text()}`);
    }
    let task = await taskResponse.json();

    let playerResponse = await fetch(`${origin}api/players/${task.player_id}`);
    if (playerResponse.ok) {
        let player = await playerResponse.json();
        return [task, player];
    } else {
        throw new Error(`Failed to fetch player: ${await playerResponse.text()}`);
    }
}