// Vulnerable: Detected possible DOM-based XSS. This occurs because a portion of the URL is being used to construct an element added directly to the page. For example, a malicious actor could send someone a link like this: http://www.some.site/page.html?default=<script>alert(document.cookie)</script> which would add the script to the page. Consider allowlisting appropriate values or using an approach which does not involve the URL.
// Pattern: {'pattern': 'document.write(<... document.location.$W ...>)'} | {'pattern': 'document.write(<... location.$W ...>)'}
function vulnerable() {
  // TODO: implement pattern match
}
