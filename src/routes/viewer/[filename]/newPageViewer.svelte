<script lang="ts">
	import type { PDFPageProxy } from 'pdfjs-dist';
	import { onMount } from 'svelte';
	import type { MouseEventHandler, WheelEventHandler } from 'svelte/elements';
	import { inview } from 'svelte-inview';
	import type { Options } from 'svelte-inview';
	import type { RenderParameters } from 'pdfjs-dist/types/src/display/api';
	import type { PageViewport } from 'pdfjs-dist/types/src/display/display_utils';
	export let Ppage: Promise<PDFPageProxy>;
	export let page_number: number;

	const INITAL_PAGES_RENDERED = 2;

	let pdfViewport: PageViewport;
	let canvas0: HTMLCanvasElement;
	let canvas1: HTMLCanvasElement;
	const canvases: HTMLCanvasElement[] = [];
	let canvasContexts: CanvasRenderingContext2D[];
	let renderCanvas: number = 0;
	let page: PDFPageProxy;
	let height: number = 100;
	let width: number;
	let windowHeight: number;
	let zoom = 1;
	let rendering = false;
	let isInview: boolean;
	let offsetX: number = 0;
	let offsetY: number = 0;

	const options: Options = {
		rootMargin: '100%'
	};

	onMount(async () => {
		page = await Ppage;
		pdfViewport = (await Ppage).getViewport({ scale: 1 });
		canvases.push(canvas0);
		canvases.push(canvas1);
		canvasContexts = canvases.map((c) => c.getContext('2d') as CanvasRenderingContext2D);
		height = Math.round((pdfViewport.height / pdfViewport.width) * width);
		if (page_number <= INITAL_PAGES_RENDERED) {
			render_page(width);
		}

		options.rootMargin = `${windowHeight}px`;
	});

	const render_page = async (width: number, noCache = false) => {
		const canvas_num = (renderCanvas + 1) % 2;
		const canvas = canvases[canvas_num];
		const canvasContext = canvasContexts[canvas_num];
		// Avoid rerender by only rerender when width changes
		if ((canvas.width === width && !noCache) || rendering) {
			return;
		}
		rendering = true;
		canvas.style.width = `${width}px`;
		canvas.style.height = `${Math.round((pdfViewport.height / pdfViewport.width) * width)}px`;
		canvas.width = width * (window.devicePixelRatio || 1);
		canvas.height =
			(pdfViewport.height / pdfViewport.width) * width * (window.devicePixelRatio || 1);
		// Render PDF page into canvas context
		const transform = [devicePixelRatio, 0, 0, devicePixelRatio, 0, 0];
		let renderContext: RenderParameters = {
			canvasContext,
			viewport: page.getViewport({
				scale: (width / pdfViewport.width) * zoom,
				offsetX: offsetX,
				offsetY: offsetY
			}),
			background: 'rgba(255, 255, 255, 0)',
			transform
		};
		page.render(renderContext).promise.then(() => {
			rendering = false;
			canvas.style.visibility = 'visible';
			canvases[(canvas_num + 1) % 2].style.visibility = 'hidden';
			renderCanvas = (renderCanvas + 1) % 2;
		});
	};

	const onZoom: WheelEventHandler<HTMLDivElement> = (event) => {
		if (event.ctrlKey) {
			let scale_vector = event.deltaY ?? 0;
			let new_zoom = zoom;
			new_zoom += 0.005 * -scale_vector;
			new_zoom = Math.abs(new_zoom);
			new_zoom = new_zoom < 0.4 ? 0.5 : new_zoom;
			new_zoom = new_zoom > 5 ? 5 : new_zoom;
			zoom = new_zoom;
			render_page(width, true);
		}
	};

	const onMouse: MouseEventHandler<HTMLDivElement> = (event) => {
		if (event.buttons === 1) {
			offsetX += event.movementX;
			offsetY += event.movementY;
			render_page(width, true);
		}
	};

	const onResize = (width: number) => {
		if (isInview) {
			render_page(width);
		}
	};

	$: onResize(width);
</script>

<div
	class="wrapper"
	bind:clientWidth={width}
	use:inview={options}
	on:inview_enter={() => {
		render_page(width);
		isInview = true;
	}}
	on:inview_leave={() => {
		isInview = false;
	}}
	on:wheel={onZoom}
	on:mousemove={onMouse}
	style="height: {height}px;"
>
	<canvas class="canvas" bind:this={canvas0} bind:clientHeight={height} />
	<canvas class="canvas" bind:this={canvas1} style="visibility: hidden;" />
</div>

<style>
	.canvas {
		position: absolute;
		top: 0;
		left: 0;
		border-radius: 10px;
		background: rgba(255, 255, 255, 0.8);
	}
	.wrapper {
		position: relative;
		display: block;
		width: 100%;
	}
	.wrapper:nth-last-child(1) {
		margin-bottom: 0;
	}
</style>
