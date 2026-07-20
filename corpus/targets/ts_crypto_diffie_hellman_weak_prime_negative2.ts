// SAFE alternative: X25519 ECDH key agreement
import { createECDH } from 'node:crypto';

function createX25519Keypair(): { publicKey: Buffer; privateKey: Buffer } {
  const ecdh = createECDH('x25519');
  ecdh.generateKeys();
  return {
    publicKey: ecdh.getPublicKey(),
    privateKey: ecdh.getPrivateKey(),
  };
}

function computeSharedSecret(theirPublic: Buffer, myPrivate: Buffer): Buffer {
  const ecdh = createECDH('x25519');
  ecdh.setPrivateKey(myPrivate);
  return ecdh.computeSecret(theirPublic);
}
