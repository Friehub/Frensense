// Vulnerable: This 'zypper install' is missing the '-y' switch. This might stall builds because it requires human intervention. Add the '-y' switch.
// Pattern: RUN ... zypper install ...
function vulnerable() {
  // TODO: implement pattern match
}
