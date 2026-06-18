// Vulnerable: Avoid using sudo in Dockerfiles. Running processes as a non-root user can help  reduce the potential impact of configuration errors and security vulnerabilities.
// Pattern: RUN sudo ...
function vulnerable() {
  // TODO: implement pattern match
}
