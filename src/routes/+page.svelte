<script lang="ts">
	import { goto } from '$app/navigation';
	import { open as openDialog } from '@tauri-apps/api/dialog';

	const readFile = async () => {
		let filename = await openDialog({
			multiple: false,
			title: 'Open PDF',
			filters: [
				{
					name: 'PDF',
					extensions: ['pdf']
				}
			]
		});

		if (filename) {
			await openFile(filename.toString());
		}
	};

	const openFile = async (filename: string) => {
		goto(`/viewer/${encodeURIComponent(filename)}`);
	};
</script>

<main class="column">
	<div class="row">
		<h1>Welcome to CleanDocs!</h1>
	</div>
	<div class="row">
		<p>A clean PDF Reader for MacOS!</p>
	</div>
	<div class="row">
		<button class="button" on:click={readFile}>Open PDF</button>
	</div>
</main>

<style>
	.row {
		display: flex;
		flex-direction: row;
		justify-content: space-around;
	}

	.column {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
	}

	.button {
		text-decoration: none;
		text-align: center;
		border: none;
		padding: 10px 20px;
		border-radius: 5px;
	}
</style>
