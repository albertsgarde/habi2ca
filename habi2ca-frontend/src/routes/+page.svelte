<script lang="ts">
	import { expect, origin } from '$lib/base';
	import { addXp, type Player } from '$lib/player';
	import { completeTask, createTask, getTasks, type Task } from '$lib/task';

	export let data: { player: Player; tasks: Task[] };

	$: player = data.player;
	$: tasks = data.tasks;
</script>

<h1>Habi2ca</h1>

<button
	on:click={async () => {
		let originUrl = expect($origin, 'apiOrigin should exist once page is loaded.');
		await createTask(originUrl, {
			player: player.id,
			name: 'New Task',
			description: 'This is a new task.',
			completed: false
		});
		tasks = await getTasks(originUrl, player.id);
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
