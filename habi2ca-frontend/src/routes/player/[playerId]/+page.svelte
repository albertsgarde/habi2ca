<script lang="ts">
	import { expect, origin } from '$lib/base';
	import { addXp, type Player } from '$lib/player';
	import { completeTask, getTasks, type Task } from '$lib/task';
	import Title from '$lib/Title.svelte';
	import TaskCreationDialog from './TaskCreationDialog.svelte';

	export let data: { player: Player; tasks: Task[] };

	$: player = data.player;
	$: tasks = data.tasks;

	let showCreateTaskDialog = false;
	let createTaskDialog: TaskCreationDialog;
</script>

<Title />

<button
	on:click={async () => {
		showCreateTaskDialog = true;
	}}
>
	Create Task
</button>
<p>Name: {player.name}</p>
<p>Level: {player.level}</p>
<p>XP: {player.xp}/{player.xp_requirement}</p>
<button
	on:click={async () =>
		(player = await addXp(
			expect($origin, 'apiOrigin should exist once page is loaded.'),
			player.id
		))}>Add XP</button
>
{#each tasks as { id, completed, name, description }}
	{#if !completed}
		<div class="task-card">
			<h3>{name}</h3>
			<p>{description}</p>
			<button
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
