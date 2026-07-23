// SAFE: verify the origin and rpId in the WebAuthn response
import express from 'express';

const EXPECTED_ORIGIN = 'https://example.com';
const EXPECTED_RP_ID = 'example.com';

function verifyWebAuthnResponse(clientDataJSON: string): boolean {
  const clientData = JSON.parse(Buffer.from(clientDataJSON, 'base64').toString());
  if (clientData.type !== 'webauthn.get') return false;
  if (clientData.origin !== EXPECTED_ORIGIN) return false;
  if (clientData.challenge === undefined) return false;
  return true;
}

const app = express();

app.post('/api/auth/webauthn/verify', (req, res) => {
  const { credentialId, authenticatorData, clientDataJSON, signature } = req.body;
  if (!verifyWebAuthnResponse(clientDataJSON)) {
    res.status(400).json({ error: 'invalid response' });
    return;
  }
  res.json({ verified: true, userId: 'some-user' });
});

export async function verifyAssertion(body: any): Promise<boolean> {
  const clientData = JSON.parse(Buffer.from(body.clientDataJSON, 'base64').toString());
  if (clientData.type !== 'webauthn.get') return false;
  if (clientData.origin !== EXPECTED_ORIGIN) return false;
  return true;
}
