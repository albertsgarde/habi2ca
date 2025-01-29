<script lang="ts">
	import { goto } from '$app/navigation';
	import PlayerCreationDialog from './PlayerCreationDialog.svelte';
	import type { PlayerInfo } from './playerInfo';

	export let data: { players: PlayerInfo[] };

	$: playerInfos = data.players;

	let showCreatePlayerDialog = false;
	let createPlayerDialog: PlayerCreationDialog;
</script>

<button
	type="button"
	class="btn variant-filled-surface"
	on:click={() => {
		showCreatePlayerDialog = true;
	}}>Create Player</button
>

{#each playerInfos as { player, numTasks, numHabits }}
	<div>
		<h2 class="h2">
			{player.name}
			<button
				type="button"
				class="btn variant-filled-surface"
				on:click={() => goto(`/players/${player.id}`)}>Play!</button
			>
		</h2>
		<p>Level: {player.level}</p>
		<p>XP: {player.xp}/{player.xp_requirement}</p>
		<p>Number of habits: {numHabits}</p>
		<p>Number of tasks: {numTasks}</p>
	</div>
{/each}

<PlayerCreationDialog
	bind:this={createPlayerDialog}
	bind:showModal={showCreatePlayerDialog}
	update={async (newPlayerInfo) => {
		playerInfos = [...playerInfos, newPlayerInfo];
	}}
></PlayerCreationDialog>
