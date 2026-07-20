// SAFE: only store credentials in response to a user gesture
let credentialSavePending = false;

export function onUserAcceptedSave(): void {
  credentialSavePending = true;
}

export async function saveCredentials(username: string, password: string): Promise<void> {
  if (!credentialSavePending) {
    throw new Error('User must consent to credential storage');
  }
  const cred = new PasswordCredential({
    id: username,
    password: password,
    name: 'Login for ' + username,
  });
  await navigator.credentials.store(cred);
  credentialSavePending = false;
}

export async function autoSaveAfterLogin(username: string, serverPassword: string): Promise<void> {
  if (!credentialSavePending) return;
  const credential = new PasswordCredential({
    id: username,
    password: serverPassword,
  });
  await navigator.credentials.store(credential);
  credentialSavePending = false;
}
