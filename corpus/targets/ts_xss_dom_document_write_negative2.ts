// SAFE: Avoids document.write; instead uses innerHTML with sanitization
import DOMPurify from "dompurify";

export function renderAd() {
    const adContent = location.search.slice(1);
    const div = document.createElement("div");
    div.className = "ad";
    div.innerHTML = DOMPurify.sanitize(adContent);
    document.body.appendChild(div);
}

export function renderTrackingPixel() {
    const pixelUrl = new URLSearchParams(location.search).get("pixel");
    if (pixelUrl && pixelUrl.startsWith("https://")) {
        const img = document.createElement("img");
        img.src = pixelUrl;
        img.width = 1;
        img.height = 1;
        document.body.appendChild(img);
    }
}
