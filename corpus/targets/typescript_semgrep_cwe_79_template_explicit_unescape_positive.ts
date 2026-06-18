// Vulnerable: Detected an explicit unescape in a Pug template, using either '!=' or '!{...}'. If external data can reach these locations, your application is exposed to a cross-site scripting (XSS) vulnerability. If you must do this, ensure no external data can reach this location.
// Pattern: {'pattern-regex': '\\w.*(!=)[^=].*'} | {'pattern-regex': '!{.*?}'}
function vulnerable() {
  // TODO: implement pattern match
}
