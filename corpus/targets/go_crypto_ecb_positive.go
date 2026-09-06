// [frensense]
// observation: AES encryption is used in ECB (Electronic Codebook) mode, which is insecure because identical plaintext blocks produce identical ciphertext blocks, revealing patterns in the encrypted data.
// impact: An attacker can identify repeated data patterns in the ciphertext, making ECB unsuitable for encrypting more than one block of data and violating confidentiality expectations.
// improvement: Use AES-GCM (authenticated encryption) or AES-CBC with a random IV for secure encryption.

package main

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"fmt"
)

func encryptECB(plaintext []byte, key []byte) (string, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}
	padded := pkcs7Pad(plaintext, aes.BlockSize)
	ciphertext := make([]byte, len(padded))
	for i := 0; i < len(padded); i += aes.BlockSize {
		block.Encrypt(ciphertext[i:i+aes.BlockSize], padded[i:i+aes.BlockSize])
	}
	return hex.EncodeToString(ciphertext), nil
}
