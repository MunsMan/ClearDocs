import type { PageViewport, PDFPageProxy } from "pdfjs-dist";
import type { RenderParameters } from "pdfjs-dist/types/src/display/api";
import { LinkTarget, PDFLinkService } from 'pdfjs-dist/web/pdf_viewer';
import * as PDFJS from 'pdfjs-dist';
import { invoke } from "@tauri-apps/api";

export interface PageContainer {
    canvases: [HTMLCanvasElement, HTMLCanvasElement]
    currentCanvas: number;
    canvasContexts: [CanvasRenderingContext2D, CanvasRenderingContext2D]
    renderLock: boolean;
    pdfViewport: PageViewport;
    zoom: number;
    pageWidth: number;
    pageSetting: PageSettings;
    pdfPage: PDFPageProxy;
    textLayer: HTMLDivElement;
    annotationLayer: HTMLDivElement;
    isInview: boolean;
    updateSettings: (pageSettings: PageSettings) => void;
    isHighRes: boolean;
}

export interface PageSettings {
    scaleFactor: number;
    offsetX: number;
    offsetY: number;
}


const devicePixelRatio = (window.devicePixelRatio || 1)
const transform = [devicePixelRatio, 0, 0, devicePixelRatio, 0, 0];

export const render_page = async (page: PageContainer, prerender = true, no_cache = false) => {
    const resolution = prerender ? 0.5 : devicePixelRatio;
    const canvas_num = (page.currentCanvas + 1) % 2;
    const canvas = page.canvases[canvas_num];
    const canvasContext = page.canvasContexts[canvas_num];
    if (prerender && page.isHighRes) {
        return;
    }
    // Avoid rerender by only rerender when width changes
    if ((canvas && canvas.width === page.pageWidth && !no_cache) || page.renderLock) {
        return;
    }
    page.renderLock = true;
    canvas.style.width = `${page.pageWidth}px`;
    canvas.style.height = `${Math.round((page.pdfViewport.height / page.pdfViewport.width) * page.pageWidth)}px`;
    canvas.width = page.pageWidth * resolution;
    canvas.height =
        (page.pdfViewport.height / page.pdfViewport.width) * page.pageWidth * resolution;
    page.pageSetting.scaleFactor = (page.pageWidth / page.pdfViewport.width) * page.zoom * resolution / devicePixelRatio;
    page.updateSettings(page.pageSetting);
    // Render PDF page into canvas context
    let renderContext: RenderParameters = {
        canvasContext,
        viewport: page.pdfPage.getViewport({
            scale: page.pageSetting.scaleFactor,
            offsetX: page.pageSetting.offsetX,
            offsetY: page.pageSetting.offsetY
        }),
        background: 'rgba(255, 255, 255, 0)',
        transform
    };
    page.pdfPage.render(renderContext).promise.then(
        async () => {
            await Promise.all([
                switchCanvases(page),
                renderTextLayer(page),
                renderAnnotationLayer(page)
            ])
            if (!prerender) {
                page.isHighRes = true
            }
            page.renderLock = false;
            page.currentCanvas = (page.currentCanvas + 1) % 2;
            if (page.isInview && prerender) {
                console.log("Prerendering finished, now starting real render")
                setTimeout(() => render_page(page, false, false), 100)
            } else {
                console.log(`Not in view! Page:${page.pdfPage.pageNumber}`)
            }
        }
    );
};

const getRenderContext = (page: PageContainer): RenderParameters => (
    {
        canvasContext: page.canvasContexts[page.currentCanvas],
        viewport: page.pdfPage.getViewport({
            scale: page.pageSetting.scaleFactor,
            offsetX: page.pageSetting.offsetX,
            offsetY: page.pageSetting.offsetY
        }),
        background: 'rgba(255, 255, 255, 0)',
        transform
    }
)

const switchCanvases = (page: PageContainer) => {
    const currentCanvas = (page.currentCanvas + 1) % 2;
    page.canvases[currentCanvas].style.visibility = 'visible';
    page.canvases[(currentCanvas + 1) % 2].style.visibility = 'hidden';
}

const renderTextLayer = (page: PageContainer) => {
    page.pdfPage.getTextContent().then((textContent) => {
        PDFJS.renderTextLayer({
            textContentSource: textContent,
            container: page.textLayer,
            viewport: getRenderContext(page).viewport,
            textDivs: []
        });
    });
}

const renderAnnotationLayer = async (page: PageContainer) => {
    const annotations = await page.pdfPage.getAnnotations()
    const linkService = new PDFLinkService();
    // The LinkTarget is currently wrong and we need our own handler
    linkService.externalLinkTarget = LinkTarget.TOP;

    PDFJS.AnnotationLayer.render({
        viewport: getRenderContext(page).viewport,
        div: page.annotationLayer,
        annotations,
        page: page.pdfPage,
        downloadManager: undefined,
        linkService,
        renderForms: false
    })
    const links = Array.from(page.annotationLayer.getElementsByTagName('a'));
    links
        .filter((link) => !link.href.startsWith('http://localhost'))
        .map((link) => {
            const address = link.href;
            link.href = '';
            link.onclick = () => {
                invoke('open_external_link', { link: address });
            };
        });
    links
        .filter((link) => link.href.startsWith('http://localhost'))
        .map((link) => {
            const address = link.href;
            link.href = '';
            link.onclick = () => {
                console.log('GoTo internal');
                console.log(address.split('#')[1]);
            };
        });
}
