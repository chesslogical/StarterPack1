# 14-vec-u8-encrypt
note rust10 is better! 
[dependencies]
zeroize = "1.5.0"




better version of 8

The code is a simple command-line application written in Rust that takes a file name and password from the user, reads the contents of 
the specified file, encrypts or decrypts the contents using the XOR cipher algorithm with the provided password, and writes the result
back to the file. The code also uses the zeroize crate to zeroize sensitive data (such as the file name, password, and file contents) 
from memory after they are no longer needed.

Here's how the code works in more detail:

The program prompts the user to enter a file name and password using the println! macro and the read_input_line function. 
The read_input_line function reads a line of input from the console using the std::io module and returns it as a String.

The program trims any whitespace from the file name and password using the trim method. This ensures that there are no leading or
trailing spaces in the input, which could cause issues when opening or creating files.

The program opens the specified file using the File::open method from the std::fs module and reads its contents into a Vec<u8> using 
the read_to_end method from the std::io module. The contents of the file are stored in memory as a byte array.

The program encrypts or decrypts the contents of the file using the XOR cipher algorithm with the provided password. The encrypted 
vector is used to store the result of the operation.

The program writes the encrypted or decrypted contents back to the file using the File::create method from the std::fs module and the
write_all method from the std::io module. This overwrites the original contents of the file with the new data.

The program uses the zeroize crate to zeroize sensitive data from memory after it is no longer needed. The zeroize method overwrites
the data in memory with zeros, making it more difficult for an attacker to recover the data after it has been used. This helps to prevent
data leaks and other security vulnerabilities.

The program prints the time taken for the operation using the std::time::Instant module and the elapsed method. This gives the user
an idea of how long the operation took to complete.

The program prompts the user to hit any key to exit using the println! macro and the read_input_char function. The read_input_char 
function reads a single character of input from the console using the std::io module and returns it as a char.

In terms of security, the code is relatively secure because it uses a simple XOR cipher algorithm to encrypt and decrypt the file contents.
However, XOR cipher is not considered to be a strong encryption method and can be easily broken with advanced cryptographic techniques.
Therefore, if you need to secure sensitive data, it is recommended to use a more advanced encryption algorithm, such as AES or RSA.

In terms of memory safety, the code uses the zeroize crate to zeroize sensitive data from memory after it is no longer needed. 
This helps to prevent data leaks and other security vulnerabilities. If you are building on this code, make sure that
your new code does not contain any raw pointers- IF that slipped through the compiler it could cause memory probs.
