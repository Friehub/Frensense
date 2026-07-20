// SAFE: Cookie values are sanitized before being rendered; only non-HTML characters displayed
import DOMPurify from "dompurify";

export function showCookieDebug() {
    const el = document.getElementById("debug");
    if (el) el.innerHTML = DOMPurify.sanitize(`Cookies: ${document.cookie}`);
}

export function displayUserPreference() {
    const theme = document.cookie
        .split("; ")
        .find(row => row.startsWith("theme="))
        ?.split("=")[1];
    const el = document.getElementById("theme-display");
    if (el && theme) el.innerHTML = DOMPurify.sanitize(`Current theme: ${theme}`);
}
