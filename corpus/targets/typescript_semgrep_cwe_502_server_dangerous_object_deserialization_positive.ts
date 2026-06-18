// Vulnerable: Using an arbitrary object ('$PARAMTYPE $PARAM') with Java RMI is an insecure deserialization vulnerability. This object can be manipulated by a malicious actor allowing them to execute code on your system. Instead, use an integer ID to look up your object, or consider alternative serialization schemes such as JSON.
// Pattern: interface $INTERFACE extends Remote {
  $RETURNTYPE $METHOD($PARAMTYPE $PARAM) throws RemoteException;
}
function vulnerable() {
  // TODO: implement pattern match
}
