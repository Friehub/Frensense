// Vulnerable: The Dockerfile(image) mounts docker.sock to the container which may allow an attacker already inside of the container to escape container and execute arbitrary commands on the host machine.
// Pattern: {'patterns': [{'pattern': 'VOLUME $X'}, {'metavariable-regex': {'metavariable': '$X', 'regex': '/var/run/docker.sock'}}]} | {'patterns': [{'pattern-regex': '- "/var/run/docker.sock:.*"'}, {'pattern-inside': 'volumes:\n  ...\n'}]}
function vulnerable() {
  // TODO: implement pattern match
}
