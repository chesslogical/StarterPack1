note-- this is not good for very large files (like over 1gb) unless you have a modern machine with plenty of ram. 

This Rust program allows for file encryption and decryption using the argon2 key derivation function, AES-256-GCM for encryption/decryption, and HMAC for ensuring data integrity. In othere words, respectable modern crypto standandards are used. Reccomended password length is more than 10 chars. 


AES is based on https://github.com/cthiriet/cipher The idea of my fork here is to drastically simplify and also modernize things. This version is more user friendly and simple- it overwrites the file so there are less choices to have to make. Also instead of encrypting directories, any directory can be zipped then encrypted as a single file. NOTE-- The original cthiriet/cipher is a very good app and should be used in place of this simplified version. For less formal use cases and for a very simplified use case, and for an example for those learning rust-, this fork, AES is here for use. If you are actually interested in long term use of this simplified script, feed the code to chatgpt and have the code audited and improved to suit your needs. 




AES is a robust tool designed for secure encryption and decryption of files using the AES-256-GCM encryption algorithm. Built with Rust, this application ensures both the confidentiality and integrity of your data without the need for key storage, relying instead on password-based encryption.

To use the AES application, you simply need to specify whether you want to encrypt or decrypt a file and provide the path to the file. For encryption, run the command ./aes encrypt <file_path>. You will be prompted to enter a password and confirm it. This password is used to derive a secure key using the Argon2id key derivation function. The application will then encrypt the file in place, ensuring that the original file is overwritten with the encrypted data.

For decryption, use the command ./cipher decrypt <file_path>. Enter the same password used for encryption when prompted. The application will decrypt the file in place, restoring it to its original form. It is essential to use a strong and unique password to maximize security.

The underlying mechanism of the Cipher application involves several key components. It uses the Argon2id algorithm for key derivation, which converts the user-provided password into a secure 256-bit key. This process includes the use of a randomly generated salt and multiple iterations, making it resistant to brute force attacks. The AES-256-GCM encryption algorithm is employed to ensure strong encryption and data integrity. A random nonce is generated for each encryption operation, ensuring that even if the same plaintext is encrypted multiple times with the same password, the ciphertext will be different each time. This nonce, along with the salt, is stored with the encrypted file to facilitate decryption.

In summary, the Cipher application provides a secure and efficient way to encrypt and decrypt files using state-of-the-art cryptographic techniques. Its use of AES-256-GCM ensures that your data remains confidential and tamper-proof, while the Argon2id key derivation function enhances security by generating strong encryption keys from user-provided passwords. This tool is ideal for users who need to protect sensitive information without the complexities of key management.


Dependencies:
    anyhow for error handling-
    clap for command-line argument parsing-
    rand for random number generation-
    ring for cryptographic operations-
    zeroize for securely clearing sensitive data from memory-
    rpassword for securely reading passwords from the terminal-
    tar seems unused in the current code but might be intended for future use-





    
