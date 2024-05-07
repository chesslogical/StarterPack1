


This script defines a constant named SHRED_DIR [./shred] that specifies the name of the directory where the files to be encrypted are located.
NOTE- this is ONE WAY encryption. There is NO decryption. DO NOT put anything in the shred directory that you want to save. There are NO take backs! 

1) The script defines a function named main that is the entry point of the program.
The function generates a random key using the rand crate and determines the length of the key based on 
the size of the largest file in the SHRED_DIR directory.

2) The program starts a timer using the Instant type from the std::time module to measure the execution time of the encryption process.

3) The program reads all the files in the SHRED_DIR directory using the read_dir function from the std::fs module.

4) The program loops through each DirEntry object in the iterator and checks if it represents a file using the is_file method.

5) If the object represents a file, the program reads the file's contents into memory using the read function from the std::fs module.

6) The program performs a bit-shift operation on the file's contents using the bit_shift function and encrypts the file's contents using the xor function.

7) The program writes the encrypted data back to the same file using the write_all function from the std::fs module.

8) The program keeps track of the number of files that were successfully encrypted and increments a counter for each file that is encrypted.

9) Once all the files in the SHRED_DIR directory have been processed, the program stops the timer and calculates the elapsed time of the encryption process.

10) The program prints the results of the encryption process, including the elapsed time, the number of files that were encrypted, and the number of bytes
used in the key.

11) Finally, the program waits for the user to input any character before exiting the program. The program also defines two helper functions bit_shift and xor, 
which perform the bit-shift and XOR operations, respectively, on slices of bytes.








