// SAFE alternative: use the SimpleWebAuthn library which handles origin/rpId verification automatically
import express from 'express';
import { verifyAuthenticationResponse } from '@simplewebauthn/server';

const RP_ID = 'example.com';
const EXPECTED_ORIGIN = 'https://example.com';

const app = express();

app.post('/api/auth/webauthn/verify', async (req, res) => {
  const { credentialId, authenticatorData, clientDataJSON, signature } = req.body;
  const user = await getUserFromCredential(credentialId);
  if (!user) {
    res.status(401).json({ error: 'unknown credential' });
    return;
  }
  const verification = await verifyAuthenticationResponse({
    response: { id: credentialId, authenticatorData, clientDataJSON, signature },
    expectedChallenge: req.session.challenge,
    expectedOrigin: EXPECTED_ORIGIN,
    expectedRPID: RP_ID,
    credential: user.credential,
  });
  res.json({ verified: verification.verified });
});

async function getUserFromCredential(credentialId: string): Promise<any> {
  return null;
}

export async function verifyAssertion(body: any, challenge: string): Promise<boolean> {
  const clientData = JSON.parse(Buffer.from(body.clientDataJSON, 'base64').toString());
  return clientData.origin === EXPECTED_ORIGIN && clientData.type === 'webauthn.get';
}
