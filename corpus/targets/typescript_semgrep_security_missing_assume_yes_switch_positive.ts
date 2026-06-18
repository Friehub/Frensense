// Vulnerable: This 'apt-get install' is missing the '-y' switch. This might stall builds because it requires human intervention. Add the '-y' switch.
// Pattern: RUN ... apt-get install ... $MULTIFLAG ...
function vulnerable() {
  // TODO: implement pattern match
}
