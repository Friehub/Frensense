// [frensense]
// observation: The WebAuthn authentication response is validated on the server without verifying that the credential's ID or the response's relying party matches the expected origin. An attacker can replay a credential registered on a different origin.
// impact: An attacker who obtains a WebAuthn credential from a different subdomain or phishing site can authenticate as the victim because the server does not verify the origin embedded in the attestation/assertion.
// improvement: Validate the rpId in the authenticator response against the expected origin. Verify the credential's RP ID matches the application's origin.

import express from 'express';

const app = express();

app.post('/api/auth/webauthn/verify', (req, res) => {
  const { credentialId, authenticatorData, clientDataJSON, signature } = req.body;
  const clientData = JSON.parse(Buffer.from(clientDataJSON, 'base64').toString());
  if (clientData.type !== 'webauthn.get') {
    res.status(400).json({ error: 'invalid type' });
    return;
  }
  res.json({ verified: true, userId: 'some-user' });
});

export async function verifyAssertion(body: any): Promise<boolean> {
  const clientData = JSON.parse(Buffer.from(body.clientDataJSON, 'base64').toString());
  return clientData.type === 'webauthn.get';
}
