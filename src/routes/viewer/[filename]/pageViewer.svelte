<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	export let pdf_name: string;
	export let page_num: number;

	let height: number = 0;
	let render_scale = 0;
	let image = '';
	let zoom;
	onMount(async () => renderFast());

	const invoke_render = async (scale: number, type: 'async' | 'sync') => {
		const { page } = (await invoke(`${type}_render_page`, {
			pdf_name,
			page_num,
			height: Math.round(height * scale)
		})) as { page: string };
		if (render_scale < height * scale) {
			render_scale = height * scale;
			image = page;
		}
	};
	const renderFast = async () => {
		invoke_render(0.7, 'sync');
		invoke_render(2, 'async');
	};
</script>

<div class="container">
	<img class="image" src={'data:image/png;base64,' + image} alt="pdf" bind:this={zoom} />
</div>

<svelte:window bind:innerHeight={height} />

<style>
	.container {
		position: relative;
	}
	.image {
		position: relative;
		height: 100%;
		width: 100%;
	}
</style>
