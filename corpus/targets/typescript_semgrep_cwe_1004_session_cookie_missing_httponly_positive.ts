// Vulnerable: A session cookie was detected without setting the 'HttpOnly' flag. The 'HttpOnly' flag for cookies instructs the browser to forbid client-side scripts from reading the cookie which mitigates XSS attacks. Set the 'HttpOnly' flag by setting 'HttpOnly' to 'true' in the Options struct.
// Pattern: &sessions.Options{
  ...,
}
function vulnerable() {
  // TODO: implement pattern match
}
