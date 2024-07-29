
![images](https://github.com/user-attachments/assets/50cc6a2d-fe8f-4cba-9b74-a0a2b8ded78a)


## Cha
Cha provides a robust encryption solution using the ChaCha20-Poly1305 algorithm, known for its strong security and performance. It allows users to encrypt and decrypt files securely using a password, ensuring the confidentiality and integrity of their data. The app employs standard cryptographic practices, including key derivation, nonce generation, and secure storage of necessary metadata.

How It Works
The encryption process begins with key derivation, where a cryptographic key is generated from a user-provided password using Argon2, a secure key derivation function (KDF). Argon2 is chosen for its resistance to brute-force attacks, making it ideal for password-based encryption. The key derivation process requires a salt, which is a random value that ensures the same password does not always result in the same encryption key. In this application, a 16-byte salt is generated randomly for each encryption operation, ensuring uniqueness and enhancing security. The salt is stored at the beginning of the encrypted file, allowing it to be retrieved during decryption.

The nonce, or "number used once," is another critical component in the encryption process. A 12-byte nonce is generated for each chunk of data to be encrypted. The nonce ensures that even if the same plaintext is encrypted multiple times with the same key, the resulting ciphertexts will be different. This feature protects against replay attacks and helps maintain the security of the encrypted data. Nonces are stored alongside the ciphertext within the encrypted file and do not need to be kept secret, but they must be unique for each encryption operation with the same key.

During encryption, the user provides a password, which, along with the generated salt, is used to derive a 256-bit key via Argon2. The salt is stored at the beginning of the output file, followed by the encrypted data. For each chunk of plaintext, a unique nonce is generated and used with the ChaCha20-Poly1305 algorithm to produce the ciphertext and an authentication tag. This tag is essential for verifying the integrity and authenticity of the encrypted data, ensuring that any tampering can be detected.

To decrypt a file, the application retrieves the salt and nonce from the encrypted file. The user must provide the same password used during encryption. The salt is used in conjunction with the password to derive the same cryptographic key. The nonce and key are then used to decrypt the ciphertext, producing the original plaintext if the password is correct and the data has not been tampered with. This process ensures that even if an unauthorized party gains access to the encrypted file, they cannot decrypt the data without the correct password.

The application provides a secure and straightforward method for protecting sensitive files. By leveraging the strengths of the ChaCha20-Poly1305 algorithm and the Argon2 key derivation function, it offers a high level of security while maintaining user simplicity. The storage of salt and nonce within the encrypted file ensures that all necessary information for decryption is available, assuming the user retains their password. This approach balances security and usability, making it an effective tool for encrypting personal or sensitive data.
