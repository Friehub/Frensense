// Vulnerable: A session cookie was detected without setting the 'Secure' flag. The 'secure' flag for cookies prevents the client from transmitting the cookie over insecure channels such as HTTP. Set the 'Secure' flag by setting 'Secure' to 'true' in the Options struct.
// Pattern: &sessions.Options{
  ...,
}
function vulnerable() {
  // TODO: implement pattern match
}
