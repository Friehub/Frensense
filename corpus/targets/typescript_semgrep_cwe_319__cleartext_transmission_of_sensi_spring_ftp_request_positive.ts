// Vulnerable: Checks for outgoing connections to ftp servers via Spring plugin ftpSessionFactory. FTP does not encrypt traffic, possibly leading to PII being sent plaintext over the network.
// Pattern: {'pattern': '$SF = new DefaultFtpSessionFactory(...);\n...\n$SF.setHost("=~/^[fF][tT][pP]://.*/");\n...\n$SF.$FUNC(...);\n'} | {'pattern': '$SF = new DefaultFtpSessionFactory(...);\n...\nString $URL = "=~/^[fF][tT][pP]://.*/";\n...\n$SF.setHost($URL);\n...\n$SF.$FUNC(...);\n'}
function vulnerable() {
  // TODO: implement pattern match
}
