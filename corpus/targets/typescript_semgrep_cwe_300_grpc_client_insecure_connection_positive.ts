// Vulnerable: Found an insecure gRPC connection using 'grpc.WithInsecure()'. This creates a connection without encryption to a gRPC server. A malicious attacker could tamper with the gRPC message, which could compromise the machine. Instead, establish a secure connection with an SSL certificate using the 'grpc.WithTransportCredentials()' function. You can create a create credentials using a 'tls.Config{}' struct with 'credentials.NewTLS()'. The final fix looks like this: 'grpc.WithTransportCredentials(credentials.NewTLS(<config>))'.
// Pattern: $GRPC.Dial($ADDR, ..., $GRPC.WithInsecure(...), ...)
function vulnerable() {
  // TODO: implement pattern match
}
