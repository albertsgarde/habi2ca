<script lang="ts">
	import { expect, origin } from '$lib/base';
	import { addXp, type Player } from '$lib/player';
	import { completeTask, getTasks, type Task } from '$lib/task';
	import TaskCreationDialog from './TaskCreationDialog.svelte';

	export let data: { player: Player; tasks: Task[] };

	$: player = data.player;
	$: tasks = data.tasks;

	let showCreateTaskDialog = false;
	let createTaskDialog: TaskCreationDialog;
</script>

<h1>Habi2ca</h1>

<button
	on:click={async () => {
		showCreateTaskDialog = true;
	}}
>
	Create Task
</button>
<p>Name: {player.data.name}</p>
<p>XP: {player.data.xp}</p>
<button
	on:click={async () =>
		(player = await addXp(
			expect($origin, 'apiOrigin should exist once page is loaded.'),
			player.id
		))}>Add XP</button
>
{#each tasks as { id, data }}
	{#if !data.completed}
		<div class="task-card">
			<h3>{data.name}</h3>
			<p>{data.description}</p>
			<button
				on:click={async () => {
					let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
					await completeTask(originUrl, id);
					tasks = await getTasks(originUrl, player.id);
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
