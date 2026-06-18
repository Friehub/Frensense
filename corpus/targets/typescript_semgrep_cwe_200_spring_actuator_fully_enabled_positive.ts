// Vulnerable: Spring Boot Actuator is fully enabled. This exposes sensitive endpoints such as /actuator/env, /actuator/logfile, /actuator/heapdump and others. Unless you have Spring Security enabled or another means to protect these endpoints, this functionality is available without authentication, causing a significant security risk.
// Pattern: management.endpoints.web.exposure.include=*
function vulnerable() {
  // TODO: implement pattern match
}
