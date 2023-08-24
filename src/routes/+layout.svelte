<script lang="ts">
	import { goto } from '$app/navigation';
	import { listen } from '@tauri-apps/api/event';

	interface DocumentEvent {
		url: string;
	}

	listen<DocumentEvent>('document', (event) => {
		const filename = event.payload.url;
		console.log(`Event Info: ${JSON.stringify(event, null, 4)}`);
		goto(`/viewer/${encodeURIComponent(filename)}`);
	});
</script>

<div class="main">
	<slot />
</div>

<style>
	.main {
		padding: 0;
		margin: 0;
	}
</style>
