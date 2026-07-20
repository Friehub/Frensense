// SAFE: encrypts values before persisting; decrypts on read

import { atomWithStorage } from 'jotai/utils'
import { encrypt, decrypt } from '@/lib/crypto'

const encryptStorage = {
  getItem: (key: string) => {
    const raw = localStorage.getItem(key)
    return raw ? decrypt(raw) : ''
  },
  setItem: (key: string, value: string) => {
    localStorage.setItem(key, encrypt(value))
  },
  removeItem: (key: string) => localStorage.removeItem(key),
}

export const ssnAtom = atomWithStorage<string>('user-ssn', '', encryptStorage)
