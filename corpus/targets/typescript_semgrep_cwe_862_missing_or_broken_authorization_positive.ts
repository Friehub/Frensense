// Vulnerable: Anonymous access shouldn't be allowed unless explicit by design. Access control checks are missing and potentially can be bypassed. This finding violates the principle of least privilege or deny by default, where access should only be permitted for a specific set of roles or conforms to a custom policy or users.
// Pattern: public class $CLASS : Controller {
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
