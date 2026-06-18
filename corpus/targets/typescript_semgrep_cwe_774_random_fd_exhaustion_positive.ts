// Vulnerable: Call to 'read()' without error checking is susceptible to file descriptor exhaustion. Consider using the 'getrandom()' function.
// Pattern: {'patterns': [{'pattern': '$FD = open("/dev/urandom", ...);\n...\nread($FD, ...);\n'}, {'pattern-not': '$FD = open("/dev/urandom", ...);\n...\n$BYTES_READ = read($FD, ...);\n'}]} | {'patterns': [{'pattern': '$FD = open("/dev/random", ...);\n...\nread($FD, ...);\n'}, {'pattern-not': '$FD = open("/dev/random", ...);\n...\n$BYTES_READ = read($FD, ...);\n'}]}
function vulnerable() {
  // TODO: implement pattern match
}
