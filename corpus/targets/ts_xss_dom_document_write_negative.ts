// SAFE: Uses DOM manipulation instead of document.write; content is set via textContent
export function renderAd() {
    const el = document.createElement("div");
    el.className = "ad";
    el.textContent = "Advertisement";
    document.body.appendChild(el);
}

export function renderTrackingPixel() {
    const img = document.createElement("img");
    img.src = "/tracking.gif";
    img.width = 1;
    img.height = 1;
    img.alt = "";
    document.body.appendChild(img);
}
