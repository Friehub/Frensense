// SAFE: User input is sanitized with DOMPurify before jQuery html() call
import DOMPurify from "dompurify";

export function loadComment() {
    const comment = new URLSearchParams(location.search).get("comment");
    $("#comment-box").html(DOMPurify.sanitize(comment));
}

export function showNotification() {
    const msg = location.hash.slice(1);
    $("#notifications").append(`<div class="alert">${DOMPurify.sanitize(msg)}</div>`);
}

export function prependUserContent() {
    const content = new URLSearchParams(location.search).get("content");
    $("#main").prepend(DOMPurify.sanitize(content));
}
