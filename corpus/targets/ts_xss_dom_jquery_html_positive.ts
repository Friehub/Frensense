// [frensense]
// observation: jQuery methods like .html(), .append(), and .prepend() are called with user-controlled input, which jQuery interprets as HTML.
// impact: An attacker can inject <script> elements or event handlers via jQuery's HTML parser, leading to DOM-based XSS.
// improvement: Use .text() instead of .html() when setting content from user input, or sanitize the input first.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function loadComment() {
    const comment = new URLSearchParams(location.search).get("comment");
    $("#comment-box").html(comment);
}

export function showNotification() {
    const msg = location.hash.slice(1);
    $("#notifications").append(`<div class="alert">${msg}</div>`);
}

export function prependUserContent() {
    const content = new URLSearchParams(location.search).get("content");
    $("#main").prepend(content);
}
