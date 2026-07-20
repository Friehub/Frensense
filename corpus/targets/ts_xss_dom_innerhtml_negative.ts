// SAFE: Uses textContent instead of innerHTML, preventing HTML injection
export function updateBanner() {
    const hash = location.hash.slice(1);
    document.getElementById("banner")!.textContent = hash;
}

export function showSearchResults() {
    const params = new URLSearchParams(location.search);
    const term = params.get("q");
    const el = document.getElementById("results");
    if (el && term) el.textContent = `Results for: ${term}`;
}
