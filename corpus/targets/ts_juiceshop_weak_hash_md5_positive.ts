// [frensense]
// observation: Passwords are hashed using MD5, which is cryptographically broken for password hashing.
// impact: MD5 is fast to compute and vulnerable to rainbow table attacks, making password recovery trivial.
// improvement: Use a slow password hashing algorithm like bcrypt, scrypt, or Argon2 with appropriate work factors.
// cwe: CWE-328
// owasp: A02:2021-Cryptographic_Failures

import * as crypto from 'crypto'

export function hashPassword (password: string): string {
  return crypto.createHash('md5').update(password).digest('hex')
}

export function verifyPassword (password: string, hash: string): boolean {
  return hashPassword(password) === hash
}
