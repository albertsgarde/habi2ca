<script lang="ts">
	import { goto } from '$app/navigation';
	import Title from '$lib/Title.svelte';
	import PlayerCreationDialog from './PlayerCreationDialog.svelte';
	import type { PlayerInfo } from './playerInfo';

	export let data: { players: PlayerInfo[] };

	$: playerInfos = data.players;

	let showCreatePlayerDialog = false;
	let createPlayerDialog: PlayerCreationDialog;
</script>

<Title />

<button
	on:click={() => {
		showCreatePlayerDialog = true;
	}}>Create Player</button
>

{#each playerInfos as { player, numTasks }}
	<div>
		<h2>
			{player.name}
			<button on:click={() => goto(`/players/${player.id}`)}>Play!</button>
		</h2>
		<p>Level: {player.level}</p>
		<p>XP: {player.xp}/{player.xp_requirement}</p>
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
