// Vulnerable: Checks for outgoing connections to ftp servers with the 'net/ftp' package. FTP does not encrypt traffic, possibly leading to PII being sent plaintext over the network. Instead, connect via the SFTP protocol.
// Pattern: {'pattern': "$FTP = Net::FTP.new('...')\n...\n$FTP.login\n"} | {'pattern': "Net::FTP.open('...') do |ftp|\n  ...\n  ftp.login\nend\n"}
function vulnerable() {
  // TODO: implement pattern match
}
