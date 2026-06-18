// Vulnerable: Checks for requests to http (unencrypted) sites using some of ruby's most popular REST/HTTP libraries, including httparty and restclient.
// Pattern: {'pattern': 'HTTParty.$PARTYVERB("=~/[hH][tT][tT][pP]://.*/", ...)\n'} | {'pattern': '$STRING = "=~/[hH][tT][tT][pP]://.*/"\n...\nHTTParty.$PARTYVERB($STRING, ...)\n'} | {'pattern': 'RestClient.$RESTVERB "=~/[hH][tT][tT][pP]://.*/", ...\n'}
function vulnerable() {
  // TODO: implement pattern match
}
