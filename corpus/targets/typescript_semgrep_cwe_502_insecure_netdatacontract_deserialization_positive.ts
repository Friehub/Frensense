// Vulnerable: The NetDataContractSerializer type is dangerous and is not recommended for data processing. Applications should stop using NetDataContractSerializer as soon as possible, even if they believe the data they're processing to be trustworthy. NetDataContractSerializer is insecure and can't be made secure
// Pattern: new NetDataContractSerializer();
function vulnerable() {
  // TODO: implement pattern match
}
