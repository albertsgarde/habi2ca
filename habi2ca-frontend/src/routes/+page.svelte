<script lang="ts">
	import { expect, origin } from '$lib/base';
	import { addXp, type Player } from '$lib/player';

	export let data: Player;

	$: player = data;
</script>

<h1>Habi2ca</h1>

<p>Name: {player.data.name}</p>
<p>XP: {player.data.xp}</p>
<button
	on:click={async () =>
		(player = await addXp(
			expect($origin, 'apiOrigin should exist once page is loaded.'),
			player.id
		))}>Add XP</button
>
