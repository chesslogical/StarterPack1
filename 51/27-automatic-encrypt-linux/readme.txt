
compiled in Linux. 

Encrypt or decrrypt a file! No key needed, no password needed. The app does all the work for you. Note, this is NOT for serious use. Yes, it is coded in Rust so there will not likely be memory issues, but this is example code so i did not include the extensive error handling and failsafes that a more serious encryption app should have. 

encrypt and decrypt files using the AES-GCM encryption algorithm. Here’s a detailed explanation of what each part of the app does:

1. User Input
First, the program prompts the user to enter the filename of the file they want to encrypt or decrypt. This is done using the standard input (stdin).

2. Checking Encryption Status
Next, the program checks if the file is already encrypted. It does this by attempting to open the file and reading the first 13 bytes to check if they match the "ENCRYPTEDFILE" marker. If the file is smaller than 13 bytes, or if the read operation fails for any other reason, the program assumes the file is not encrypted.

3. Encrypting or Decrypting the File
Based on whether the file is encrypted, the program proceeds to either encrypt or decrypt the file:

Encryption:
The program generates a random 256-bit key and a random 96-bit nonce, both of which are required for AES-GCM encryption.
The entire content of the file is read into memory.
The file content is encrypted using the generated key and nonce.
The program then opens the same file for writing and writes the "ENCRYPTEDFILE" marker, followed by the encryption key, the nonce, and finally the encrypted content. This overwrites the original file content.
Decryption:
The program reads the key and nonce from the file. These were written to the file during the encryption process.
It reads the remaining content of the file, which is the encrypted data.
Using the key and nonce, it attempts to decrypt the encrypted data. If any part of the key, nonce, or encrypted data has been altered since encryption, this step will fail.
If decryption is successful, the program opens the file for writing again and writes the decrypted content back to the file, overwriting the encrypted content.
4. Error Handling
Throughout the process, the program checks for various error conditions, such as file not found, file too small to be encrypted, decryption failure, etc. If any error occurs, a descriptive error message is printed to the standard error output (stderr), and the program exits with a status code of 1, indicating failure.

5. Informing the User
Finally, once the operation (encryption or decryption) is successfully completed, a message is printed to the console informing the user that the operation was successful.

This way, the user can run the application repeatedly on the same file to toggle between its encrypted and decrypted states, without needing to manually manage encryption keys or worry about accidentally corrupting the file by running the operation twice in a row. However, as mentioned before, because it’s using AES-GCM, any accidental modification of the encrypted file will make it undecryptable.








If you change one byte of the encrypted file, it can NOT be decrypted! 

Yes, there is a significant risk involved with this. AES-GCM (Galois/Counter Mode) is an authenticated encryption with associated data (AEAD) cipher, meaning it provides confidentiality, integrity, and authenticity assurances on the encrypted data. If a single byte of the encrypted data (or the associated authentication tag) is altered, the decryption will fail, resulting in an error.

This is actually a feature of secure encryption schemes, as it ensures that any tampering with the encrypted data is detected. However, it does mean that if a file becomes corrupted, even in a minor way, it will not be possible to decrypt it.

To mitigate this risk, you could implement some strategies:

Backups
Always keep backups of important data, especially before performing encryption or decryption operations. This way, if something goes wrong, you can restore from the backup.

Integrity Checks
You could implement additional integrity checks or include a hash of the original file along with the encrypted data. This would allow you to verify whether the original, unencrypted file has been tampered with before attempting to decrypt it.

User Education
Educate users on the importance of not modifying encrypted files directly, as any modifications can render the file undecryptable.

Versioning
Implement a versioning system where changes are tracked, and older versions of files can be restored if needed.

Robust Error Handling
Ensure that your application has robust error handling for decryption failures, and provide clear messages to the user on what steps to take if decryption fails.

Despite these strategies, it's important to highlight that if an encrypted file is modified, repairing it to a decryptable state is generally not possible without a backup or original copy of the file. The integrity and authenticity assurances provided by AES-GCM prevent any tampered encrypted data from being decrypted.
