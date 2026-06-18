// Vulnerable: This 'dnf install' is missing the '-y' switch. This might stall builds because it requires human intervention. Add the '-y' switch.
// Pattern: RUN ... dnf install ...
function vulnerable() {
  // TODO: implement pattern match
}
