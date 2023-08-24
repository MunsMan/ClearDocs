<script lang="ts">
	import type { PDFPageProxy } from 'pdfjs-dist';
	import { onMount } from 'svelte';
	import type { MouseEventHandler, WheelEventHandler } from 'svelte/elements';
	import { inview } from 'svelte-inview';
	import type { Options } from 'svelte-inview';
	import { render_page, type PageContainer, type PageSettings } from '$lib/render_page';
	import type { PageViewport } from 'pdfjs-dist/types/src/display/display_utils';
	export let Ppage: Promise<PDFPageProxy>;
	export let scrollSpeed: number;

	let pdfViewport: PageViewport;
	let canvas0: HTMLCanvasElement;
	let canvas1: HTMLCanvasElement;
	let canvases: [HTMLCanvasElement, HTMLCanvasElement];
	let page: PDFPageProxy;
	let width: number;
	let windowHeight: number;
	let zoom = 1;
	let isInview = false;
	let offsetX: number = 0;
	let offsetY: number = 0;
	let textLayer: HTMLDivElement;
	let annotationLayer: HTMLDivElement;
	let scaleFactor: number = 1;
	let pageContainer: PageContainer | undefined;

	const options: Options = {
		rootMargin: '200px'
	};

	const updatePageSettings = (pageSettings: PageSettings) => {
		scaleFactor = pageSettings.scaleFactor;
		offsetX = pageSettings.offsetX;
		offsetY = pageSettings.offsetY;
	};

	const createPageContainer = (
		canvases: [HTMLCanvasElement, HTMLCanvasElement],
		pdfViewport: PageViewport,
		pdfPage: PDFPageProxy,
		width: number
	): PageContainer => {
		const canvasContexts = canvases.map(
			(canvas) => canvas.getContext('2d') as CanvasRenderingContext2D
		) as [CanvasRenderingContext2D, CanvasRenderingContext2D];
		return {
			canvases,
			currentCanvas: 0,
			canvasContexts,
			renderLock: false,
			pdfViewport,
			zoom: 1,
			pageWidth: width,
			pageSetting: {
				scaleFactor,
				offsetX,
				offsetY
			},
			pdfPage,
			textLayer,
			annotationLayer,
			isInview,
			isHighRes: false,
			updateSettings: updatePageSettings
		};
	};

	onMount(async () => {
		page = await Ppage;
		pdfViewport = (await Ppage).getViewport({ scale: 1 });
		canvases = [canvas0, canvas1];
		canvas0.style.width = `${width}px`;
		canvas0.style.height = `${Math.round((pdfViewport.height / pdfViewport.width) * width)}px`;
		pageContainer = createPageContainer(canvases, pdfViewport, page, width);
		options.rootMargin = `${windowHeight}px`;
	});

	const onZoom: WheelEventHandler<HTMLDivElement> = (event) => {
		if (event.ctrlKey) {
			let scale_vector = event.deltaY ?? 0;
			let new_zoom = zoom;
			new_zoom += 0.005 * -scale_vector;
			new_zoom = Math.abs(new_zoom);
			new_zoom = new_zoom < 0.4 ? 0.5 : new_zoom;
			new_zoom = new_zoom > 5 ? 5 : new_zoom;
			zoom = new_zoom;
			if (pageContainer) {
				render_page(pageContainer, false, true);
			}
		}
	};

	const onMouse: MouseEventHandler<HTMLDivElement> = (event) => {
		if (event.buttons === 1) {
			offsetX += event.movementX;
			offsetY += event.movementY;
		}
		if (pageContainer) {
			render_page(pageContainer, false);
		}
	};

	$: {
		if (pageContainer) {
			pageContainer.isInview = isInview;
		}
		console.log(scrollSpeed);
	}
</script>

<div
	class="wrapper"
	bind:clientWidth={width}
	use:inview={options}
	on:inview_enter={() => {
		if (pageContainer) {
			console.log(`Page in View ${pageContainer.pdfPage.pageNumber}`);
			render_page(pageContainer, scrollSpeed > 50 ? true : false);
		}
		isInview = true;
	}}
	on:inview_leave={() => {
		isInview = false;
	}}
	on:wheel={onZoom}
	on:mousemove={onMouse}
>
	<canvas class="canvas" bind:this={canvas0} style="position: static" />
	<canvas class="canvas" bind:this={canvas1} style="visibility: hidden; position: absolute" />
	<div
		bind:this={textLayer}
		class="textLayer"
		style="transform: translate({offsetX}px, {offsetY}px); --scale-factor: {scaleFactor}"
	/>
	<div
		bind:this={annotationLayer}
		class="annotationLayer"
		style="transform: translate({offsetX}px, {offsetY}px); --scale-factor: {scaleFactor}"
	/>
</div>

<style>
	.canvas {
		top: 0;
		left: 0;
		border-radius: 10px;
		background: rgba(255, 255, 255, 0.8);
	}
	.wrapper {
		position: relative;
		display: block;
		width: 100%;
		margin-bottom: 10px;
	}
	.wrapper:nth-last-child(1) {
		margin-bottom: 0;
	}
</style>
