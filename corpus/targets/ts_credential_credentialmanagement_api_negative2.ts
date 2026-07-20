// SAFE alternative: use the CredentialContainer's preventSilentAccess to block silent credential storage
export async function saveCredentials(username: string, password: string): Promise<void> {
  if (!navigator.credentials.preventSilentAccess) {
    return;
  }
  await navigator.credentials.preventSilentAccess();
  const cred = new PasswordCredential({
    id: username,
    password: password,
    name: 'Login for ' + username,
  });
  await navigator.credentials.store(cred);
}

export async function autoSaveAfterLogin(username: string, serverPassword: string): Promise<void> {
  if (!('credentials' in navigator)) return;
  const credential = new PasswordCredential({
    id: username,
    password: serverPassword,
  });
  await navigator.credentials.store(credential);
}
