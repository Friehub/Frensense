// SAFE: Use a modern 2048-bit group or ECDH
import { createDiffieHellman, createECDH } from 'node:crypto';

function createSafeDH(): { prime: Buffer; generator: Buffer } {
  const dh = createDiffieHellman(2048);
  dh.generateKeys();
  return { prime: dh.getPrime(), generator: dh.getGenerator() };
}

function createECDHKey(): { publicKey: Buffer; privateKey: Buffer } {
  const ecdh = createECDH('prime256v1');
  ecdh.generateKeys();
  return { publicKey: ecdh.getPublicKey(), privateKey: ecdh.getPrivateKey() };
}
