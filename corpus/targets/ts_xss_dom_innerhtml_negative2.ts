// SAFE: Uses DOMPurify to sanitize the hash value before innerHTML assignment
import DOMPurify from "dompurify";

export function updateBanner() {
    const hash = location.hash.slice(1);
    document.getElementById("banner")!.innerHTML = DOMPurify.sanitize(hash);
}

export function showSearchResults() {
    const params = new URLSearchParams(location.search);
    const term = params.get("q");
    const el = document.getElementById("results");
    if (el && term) el.innerHTML = DOMPurify.sanitize(`Results for: ${term}`);
}
