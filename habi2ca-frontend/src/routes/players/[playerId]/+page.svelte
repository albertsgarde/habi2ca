<script lang="ts">
	import { expect, origin } from '$lib/base';
	import { getHabits, incrementHabit, type Habit } from '$lib/habit';
	import { type Player } from '$lib/player';
	import { completeTask, getTasks, type Task } from '$lib/task';
	import HabitCreationDialog from './HabitCreationDialog.svelte';
	import TaskCreationDialog from './TaskCreationDialog.svelte';

	export let data: { player: Player; tasks: Task[]; habits: Habit[] };

	$: player = data.player;
	$: tasks = data.tasks;
	$: habits = data.habits;

	let showCreateTaskDialog = false;
	let createTaskDialog: TaskCreationDialog;
	let showCreateHabitDialog = false;
	let createHabitDialog: HabitCreationDialog;
</script>

<button
	class="btn btn-blue"
	on:click={async () => {
		showCreateHabitDialog = true;
	}}
>
	Create Habit
</button>
<button
	class="btn btn-blue"
	on:click={async () => {
		showCreateTaskDialog = true;
	}}
>
	Create Task
</button>
<p>Name: {player.name}</p>
<p>Level: {player.level}</p>
<p>XP: {player.xp}/{player.xp_requirement}</p>
<h2>Habits</h2>
{#each habits as { id, name, description }}
	<div class="habit-card">
		<h3>{name}</h3>
		<p>{description}</p>
		<button
			class="btn btn-blue"
			on:click={async () => {
				let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
				let [_, updatedPlayer] = await incrementHabit(originUrl, id);
				player = updatedPlayer;
			}}>Increment</button
		>
	</div>
{/each}
<h2>Tasks</h2>
{#each tasks as { id, completed, name, description }}
	{#if !completed}
		<div class="task-card">
			<h3>{name}</h3>
			<p>{description}</p>
			<button
				class="btn btn-blue"
				on:click={async () => {
					let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
					let [_, updatedPlayer] = await completeTask(originUrl, id);
					player = updatedPlayer;
					tasks = await getTasks(originUrl, updatedPlayer.id);
				}}>Complete</button
			>
		</div>
	{/if}
{/each}
<TaskCreationDialog
	bind:this={createTaskDialog}
	bind:showModal={showCreateTaskDialog}
	playerId={player.id}
	update={async () => {
		let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
		tasks = await getTasks(originUrl, player.id);
	}}
></TaskCreationDialog>
<HabitCreationDialog
	bind:this={createHabitDialog}
	bind:showModal={showCreateHabitDialog}
	playerId={player.id}
	update={async () => {
		let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
		habits = await getHabits(originUrl, player.id);
	}}
></HabitCreationDialog>
