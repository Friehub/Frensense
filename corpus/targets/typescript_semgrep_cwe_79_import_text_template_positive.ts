// Vulnerable: When working with web applications that involve rendering user-generated  content, it's important to properly escape any HTML content to prevent  Cross-Site Scripting (XSS) attacks. In Go, the `text/template` package does  not automatically escape HTML content, which can leave your application  vulnerable to these types of attacks. To mitigate this risk, it's  recommended to use the `html/template` package instead, which provides  built-in functionality for HTML escaping. By using `html/template` to render  your HTML content, you can help to ensure that your web application is more  secure and less susceptible to XSS vulnerabilities.
// Pattern: import "$IMPORT"
function vulnerable() {
  // TODO: implement pattern match
}
