// [frensense]
// observation: Jotai atomWithStorage stores sensitive data in localStorage without encryption, making it readable via browser devtools, XSS, or physical device access
// impact: An attacker with XSS or physical access can read plaintext tokens, PII, or credit card numbers from localStorage
// improvement: Use sessionStorage for transient data, or encrypt values before persisting with atomWithStorage
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { atomWithStorage } from 'jotai/utils';

function createAtoms() {
  const ssnAtom = atomWithStorage<string>('user-ssn', '');
  const creditCardAtom = atomWithStorage<string>('cc-number', '');
  return { ssnAtom, creditCardAtom };
}
