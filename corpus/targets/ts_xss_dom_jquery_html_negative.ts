// SAFE: Uses .text() to set text content instead of .html(), avoiding HTML parsing
export function loadComment() {
    const comment = new URLSearchParams(location.search).get("comment");
    $("#comment-box").text(comment);
}

export function showNotification() {
    const msg = location.hash.slice(1);
    $("#notifications").append(`<div class="alert">${$("<div>").text(msg).html()}</div>`);
}

export function prependUserContent() {
    const content = new URLSearchParams(location.search).get("content");
    $("#main").text(content);
}
