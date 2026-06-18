// Vulnerable: Found an insecure gRPC connection. This creates a connection without encryption to a gRPC client/server. A malicious attacker could tamper with the gRPC message, which could compromise the machine.
// Pattern: {'pattern': "require('grpc');\n...\n$GRPC($ADDR,...,$CREDENTIALS.createInsecure(),...);\n"} | {'pattern': "require('grpc');\n...\nnew $GRPC($ADDR,...,$CREDENTIALS.createInsecure(),...);\n"} | {'pattern': "require('grpc');\n...\n$CREDS = <... $CREDENTIALS.createInsecure() ...>;\n...\n$GRPC($ADDR,...,$CREDS,...);"}
function vulnerable() {
  // TODO: implement pattern match
}
