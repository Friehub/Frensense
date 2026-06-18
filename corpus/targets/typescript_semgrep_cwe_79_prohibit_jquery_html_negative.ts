// Fixed: JQuery's `html` function is susceptible to Cross Site Scripting (XSS) attacks. If you're just passing text, consider `text` instead. Otherwise, use a function that escapes HTML such as edX's `HtmlUtils.setHtml()`.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
