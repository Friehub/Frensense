// SAFE: Cookie values are set via textContent, preventing HTML injection
export function showCookieDebug() {
    const el = document.getElementById("debug");
    if (el) el.textContent = `Cookies: ${document.cookie}`;
}

export function displayUserPreference() {
    const theme = document.cookie
        .split("; ")
        .find(row => row.startsWith("theme="))
        ?.split("=")[1];
    const el = document.getElementById("theme-display");
    if (el && theme) el.textContent = `Current theme: ${theme}`;
}
