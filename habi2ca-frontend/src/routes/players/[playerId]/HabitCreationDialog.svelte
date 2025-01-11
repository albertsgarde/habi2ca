<script lang="ts">
	import { expect, origin } from '$lib/base';
	import Modal from '$lib/Modal.svelte';
	import { createHabit } from '$lib/habit';

	export let showModal: boolean; // boolean
	export let playerId: number;

	export let update: () => void;

	let dialog: Modal; // HTMLDialogElement

	let habitName: string = '';
	let habitDescription: string = '';

	export function close() {
		dialog.close();
	}
</script>

<Modal bind:this={dialog} bind:showModal closeButtonText="Cancel">
	<h2 slot="header">Create Habit</h2>
	<form
		on:submit|preventDefault={async () => {
			if (habitName === '' || habitName === undefined) {
				alert('Habit name cannot be empty.');
				return;
			}
			dialog.close();
			await createHabit(expect($origin, 'apiOrigin should exist once page is loaded.'), {
				player_id: playerId,
				name: habitName,
				description: habitDescription
			});
			habitName = '';
			habitDescription = '';
			update();
		}}
	>
		<input name="habitName" type="text" bind:value={habitName} placeholder="Name..." />
		<textarea name="habitDescription" bind:value={habitDescription} placeholder="Description..." />
		<button>Create</button>
	</form>
</Modal>
