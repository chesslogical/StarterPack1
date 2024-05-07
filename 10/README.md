# Encryption 10
simple rust encryption. It asks for a file name then encrypts the file. Run it again, it decrypts the file. Extremely fast.


The encryption used in this code is a simple byte-by-byte XOR cipher. XOR (exclusive OR) is a bitwise operation that takes
two equal-length binary numbers and returns a result that is the same length as the input numbers. The XOR operation returns 
a 1 in each bit position where the corresponding bits of either but not both operands are 1s.

To encrypt the file, each byte in the file is XORed with a fixed value of 1. This is done using the byte.wrapping_add(1) operation. 
To decrypt the file, the same operation is performed, but with byte.wrapping_sub(1).
This undoes the encryption and returns the original byte.
Note that this encryption method is not secure and should not be used in situations where strong encryption is required. 
This is because the key used (the value of 1) is fixed and is known to an attacker who can easily reverse the encryption process.


The encryption used in this code is a simple byte-by-byte XOR cipher. XOR (exclusive OR) is a fundamental operation in cryptography, and XOR-based encryption schemes have been used in various applications. However, this specific encryption scheme does not have a widely recognized name since it is a very basic form of encryption that is not considered to be secure.

More complex XOR-based encryption schemes do exist, such as the XOR-based stream ciphers, which generate a pseudo-random stream of bits that are XORed with the plaintext to produce the ciphertext. However, these schemes use much more sophisticated algorithms and are designed to be more secure than the basic XOR cipher used in this code.

The program first imports the necessary modules from the standard library.
The main function collects the command-line arguments into a vector of strings using env::args().
If there are fewer than two arguments (the name of the program and the filename), the program prints an error message and exits with an error code of 1.
Otherwise, the filename is extracted from the arguments vector.
The program then opens the file using File::open and reads its contents into a byte buffer using file.read_to_end(&mut buffer)?.
The program checks whether the file has been encrypted before by checking if there is a null byte (0) in the buffer using buffer.iter().any(|&byte| byte == 0).
If the file has been encrypted before, the program decrypts it by subtracting 1 from each byte in the buffer using *byte = byte.wrapping_sub(1).
If the file has not been encrypted before, the program encrypts it by adding 1 to each byte in the buffer using *byte = byte.wrapping_add(1). A null byte (0) is then appended to the end of the buffer using buffer.push(0).
The program then writes the encrypted or decrypted byte buffer back to the file using file.write_all(&buffer)?.
Finally, the main function returns an Ok(()) value to indicate that the program completed successfully.
