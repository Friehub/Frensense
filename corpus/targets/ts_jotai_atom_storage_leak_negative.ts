// SAFE: uses sessionStorage instead of localStorage so data is cleared on tab close

import { atomWithStorage } from 'jotai/utils'

export const ssnAtom = atomWithStorage<string>('user-ssn', '', undefined, {
  getStorage: () => sessionStorage,
})
export const creditCardAtom = atomWithStorage<string>('cc-number', '', undefined, {
  getStorage: () => sessionStorage,
})
