// Vulnerable: Stacktrace information is displayed in a non-Development environment. Accidentally disclosing sensitive stack trace information in a production environment aids an attacker in reconnaissance and information gathering.
// Pattern: $APP.UseDeveloperExceptionPage(...);
function vulnerable() {
  // TODO: implement pattern match
}
