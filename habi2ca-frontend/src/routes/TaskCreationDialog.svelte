<script lang="ts">
	import { expect, origin } from '$lib/base';
	import Modal from '$lib/Modal.svelte';
	import { createTask } from '$lib/task';

	export let showModal: boolean; // boolean
	export let playerId: number;

	export let update: () => void;

	let dialog: Modal; // HTMLDialogElement

	let taskName: string;
	let taskDescription: string;

	export function close() {
		dialog.close();
	}
</script>

<Modal bind:this={dialog} bind:showModal>
	<h2 slot="header">Create Task</h2>
	<form
		on:submit|preventDefault={async () => {
			if (taskName === '' || taskName === undefined) {
				alert('Task name cannot be empty.');
				return;
			}
			dialog.close();
			await createTask(expect($origin, 'apiOrigin should exist once page is loaded.'), {
				player: playerId,
				name: taskName,
				description: taskDescription,
				completed: false
			});
			taskName = '';
			taskDescription = '';
			update();
		}}
	>
		<input name="taskName" type="text" bind:value={taskName} placeholder="Name..." />
		<textarea name="taskDescription" bind:value={taskDescription} placeholder="Description..." />
		<button>Create</button>
	</form>
</Modal>
