<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import type { PDFDocumentProxy } from 'pdfjs-dist/types/src/display/api';
	import * as PDFJS from 'pdfjs-dist';
	import NewPageViewer from './newPageViewer.svelte';
	import { readBinaryFile } from '@tauri-apps/api/fs';
	import { goto } from '$app/navigation';
	import { keyboard_shortcuts } from '$lib/shortcuts';

	PDFJS.GlobalWorkerOptions.workerSrc = new URL(
		'pdfjs-dist/build/pdf.worker.js',
		import.meta.url
	).toString();

	export let data: PageData;
	let height: number = 0;
	let lastHeight: number = 0;
	let pdf: Promise<PDFDocumentProxy> | null;
	let lastScrollHeight = 0;
	let scrollSpeed = 0;

	listen('open', () => {
		goto('/');
	});

	onMount(async () => {
		console.log(data.filename);
		let file = await readBinaryFile(data.filename);
		pdf = PDFJS.getDocument(file).promise;
	});

	const onResize = (event: UIEvent & { currentTarget: EventTarget & Window }) => {
		if (event.currentTarget.innerHeight > lastHeight) {
			lastHeight = event.currentTarget.innerHeight;
		}
	};

	let pages: HTMLDivElement;
</script>

<div class="container" bind:clientHeight={height}>
	<div
		class="scroll"
		bind:this={pages}
		on:scroll={(event) => {
			scrollSpeed = Math.abs(event.target.scrollTop - lastScrollHeight);
			console.log(event.target.scrollTop, scrollSpeed);
			lastScrollHeight = event.target.scrollTop;
		}}
	>
		{#if pdf}
			{#await pdf then pdf}
				{#each [...Array(pdf.numPages).keys()] as page_number}
					<NewPageViewer Ppage={pdf.getPage(page_number + 1)} {scrollSpeed} />
				{/each}
			{/await}
		{/if}
	</div>
</div>

<svelte:window on:resize={onResize} on:keypress={(event) => keyboard_shortcuts(event, pages)} />

<link rel="stylesheet" href="../../../../node_modules/pdfjs-dist/web/pdf_viewer.css" />

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
	::-webkit-scrollbar {
		width: 0px;
		background: transparent; /* make scrollbar transparent */
	}
</style>
