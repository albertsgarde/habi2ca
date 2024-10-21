<script lang="ts">
	import { expect, origin } from '$lib/base';
	import Modal from '$lib/Modal.svelte';
	import { createPlayer } from '$lib/player';
	import type { PlayerInfo } from './playerInfo';

	export let showModal: boolean; // boolean

	export let update: (newPlayerInfo: PlayerInfo) => void;

	let dialog: Modal; // HTMLDialogElement

	let playerName: string = '';

	export function close() {
		dialog.close();
	}
</script>

<Modal bind:this={dialog} bind:showModal closeButtonText="Cancel">
	<h2 slot="header">Create Player</h2>
	<form
		on:submit|preventDefault={async () => {
			if (playerName === '' || playerName === undefined) {
				alert('Player name cannot be empty.');
				return;
			}
			dialog.close();
			const player = await createPlayer(
				expect($origin, 'apiOrigin should exist once page is loaded.'),
				playerName
			);
			playerName = '';
			update({ player, numTasks: 0 });
		}}
	>
		<input name="playerName" type="text" bind:value={playerName} placeholder="Name..." />
		<button>Create</button>
	</form>
</Modal>
