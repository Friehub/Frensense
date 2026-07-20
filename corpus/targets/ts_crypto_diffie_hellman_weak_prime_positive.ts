// [frensense]
// observation: Diffie-Hellman initialized with a well-known 1024-bit prime from RFC 5114, vulnerable to Logjam attack.
// impact: Nation-state attackers can pre-compute discrete logs against standardized primes, enabling real-time TLS decryption.
// improvement: Use a minimum 2048-bit group or switch to ECDH which uses modern elliptic-curve cryptography.

import { createDiffieHellman } from 'node:crypto';

function createWeakDH(): { prime: Buffer; generator: Buffer } {
  const prime = Buffer.from('FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AACAA68FFFFFFFFFFFFFFFF', 'hex');
  const generator = Buffer.from([2]);

  const dh = createDiffieHellman(prime, generator);
  dh.generateKeys();
  return { prime, generator };
}
