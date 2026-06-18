// Vulnerable: $METHOD is a state-changing MVC method that does not validate the antiforgery token or do strict content-type checking. State-changing controller methods should either enforce antiforgery tokens or do strict content-type checking to prevent simple HTTP request types from bypassing CORS preflight controls.
// Pattern: [$HTTPMETHOD]
public IActionResult $METHOD(...){
    ...
}
function vulnerable() {
  // TODO: implement pattern match
}
