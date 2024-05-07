# 12 AES key maker



[dependencies]
rand = "0.8.4"

The Rust code generates a random 256-bit (32-byte) key for the Advanced Encryption Standard (AES) cipher.

AES is a symmetric encryption algorithm that can use different key sizes, including 128-bit, 192-bit, and 256-bit keys. 
In this case, the code generates a 256-bit key by creating a 32-byte buffer and filling it with random bytes using a cryptographically 
secure random number generator.

A 256-bit key provides a high level of security for most practical purposes, and is recommended for applications that require strong encryption. 
The AES algorithm is widely used in a variety of applications, including secure communications, file encryption, and database encryption, among others.


First, the code imports the RngCore trait and the thread_rng function from the rand crate, as well as the File struct and io::prelude module from the 
Rust standard library.

Next, the code creates a 32-byte buffer to hold the AES key. This buffer is represented by the key variable, which is declared as a mutable array of 32
unsigned 8-bit integers ([0u8; 32]).

Then, the code generates a random AES key and fills the buffer with it using the fill_bytes method on the ThreadRng object returned by thread_rng(). 
This ensures that the key is generated using a cryptographically secure random number generator.

After the key is generated, the code creates a new file called key.key in the current directory using the File::create method. If the file cannot be 
created for any reason, the code will exit with an error message printed to the console.

Once the file has been created, the code writes the AES key to the file using the write_all method on the File object. This method writes the entire 
contents of the buffer to the file.

If the write operation fails for any reason (e.g. if the file system is full or the file is locked by another process), the code will exit with an error 
message printed to the console.

Finally, the code prints a message to the console to indicate that the key has been written to the file.

Overall, this code generates a cryptographically secure AES key and saves it to a file in the current directory. This is a useful starting point 
for building more complex cryptographic applications that require a random AES key.
