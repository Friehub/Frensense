// [frensense]
// observation: The application uses navigator.credentials.store() to save user credentials without requiring a prior user gesture or confirmation dialog. The Credential Management API allows silent credential storage, which can be abused by a script to persist attacker-controlled credentials.
// impact: An attacker who achieves script execution on the page can silently store a malicious credential via navigator.credentials.store(), and the next time the user visits, the browser may auto-fill the attacker-controlled credential, revealing the password to the attacker's server.
// improvement: Require a user gesture (e.g., button click) before calling navigator.credentials.store(). Display a confirmation UI and await user consent.

export async function saveCredentials(username: string, password: string): Promise<void> {
  const cred = new PasswordCredential({
    id: username,
    password: password,
    name: 'Login for ' + username,
  });
  await navigator.credentials.store(cred);
}

export async function autoSaveAfterLogin(username: string, serverPassword: string): Promise<void> {
  const credential = new PasswordCredential({
    id: username,
    password: serverPassword,
  });
  navigator.credentials.store(credential);
}
