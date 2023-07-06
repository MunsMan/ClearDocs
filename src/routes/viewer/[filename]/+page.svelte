<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import type { PDFDocumentProxy } from 'pdfjs-dist/types/src/display/api';
	import * as pdfjs from 'pdfjs-dist';
	import NewPageViewer from './newPageViewer.svelte';
	import { readBinaryFile } from '@tauri-apps/api/fs';
	pdfjs.GlobalWorkerOptions.workerSrc = new URL(
		'pdfjs-dist/build/pdf.worker.js',
		import.meta.url
	).toString();

	export let data: PageData;
	let height: number = 0;
	let lastHeight: number = 0;
	let pdf: Promise<PDFDocumentProxy> | null;

	onMount(async () => {
		console.log(data.filename);
		let file = await readBinaryFile(data.filename);
		pdf = pdfjs.getDocument(file).promise;
	});

	const onResize = (event: UIEvent & { currentTarget: EventTarget & Window }) => {
		if (event.currentTarget.innerHeight > lastHeight) {
			lastHeight = event.currentTarget.innerHeight;
		}
	};
</script>

<div class="container" bind:clientHeight={height}>
	<div class="scroll">
		{#if pdf}
			{#await pdf then pdf}
				{#each [...Array(pdf.numPages).keys()] as page_number}
					<NewPageViewer Ppage={pdf.getPage(page_number + 1)} {page_number} />
				{/each}
			{/await}
		{/if}
	</div>
</div>

<svelte:window
	on:resize={onResize}
	bind:innerHeight={height}
	on:keypress={(event) => console.log(event)}
/>

<style>
	.container {
		display: flex;
		height: 100vh;
		max-height: 100vh;
		width: 100%;
		flex-direction: row;
		justify-content: space-around;
	}
	.scroll {
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		overflow-y: scroll;
		overflow-x: hidden;
		width: 100vw;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	.padding {
		height: 20vh;
		width: 100%;
	}
	::-webkit-scrollbar {
		width: 0px;
		background: transparent; /* make scrollbar transparent */
	}
</style>
