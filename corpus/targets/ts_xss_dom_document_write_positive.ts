// [frensense]
// observation: User-controlled input from the query string is written directly to the page using document.write, which parses the string as HTML.
// impact: An attacker can inject arbitrary HTML/script tags via a crafted URL. When the page loads, document.write replaces the page content with attacker-controlled HTML.
// improvement: Avoid document.write entirely; use DOM manipulation methods like textContent or createElement.

export function renderAd() {
    const adContent = location.search.slice(1);
    document.write(`<div class="ad">${adContent}</div>`);
}

export function renderTrackingPixel() {
    const pixelUrl = new URLSearchParams(location.search).get("pixel");
    document.write(`<img src="${pixelUrl}" width="1" height="1" />`);
}
