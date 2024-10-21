<script lang="ts">
	import { goto } from '$app/navigation';
	import { BACKEND_ORIGIN } from '$lib/base';
	import type { PlayerInfo } from './playerInfo';

	export let data: { players: PlayerInfo[] };

	const playerInfos = data.players;
</script>

<h1>Habi2ca</h1>

{#each playerInfos as { player, numTasks }}
	<div>
		<h2>
			{player.name}
			<button on:click={() => goto(`/player/${player.id}`)}>Play!</button>
		</h2>
		<p>XP: {player.xp}</p>
		<p>Number of tasks: {numTasks}</p>
	</div>
{/each}
