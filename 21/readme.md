 Put a whole bunch of files in the /shred directory. Put subdirectory folders in other 
subdirectory folders. View the properties of the /shred folder. It said 1080 files. I ran this, it took longer than I expected, I let it 
keep going. At the end it returned that yes, it processed 1080 files. All files that were buried in sub-directories were also processed. Overall, 
this version is very good, I am surprised it goes slow for large amounts of files and subdirectories. That would be fixed with an easier encryption process
or improved memory functions. Not EVERY rust coded app can do anything instantly, but it usually does things very well. Anti-virus may have slowed 
down operations also. But as long as this is reliable, does its job and closes out without messing up system memory, I will forgive it 
for not being able to instantly process 1080 files buried deep into subdirectories. FYI the total /shred dir size was 668mb , 1080 files deep into subdirectories within subdirectories within subdirectories were processed right. It returned 17760ms for the time, and the bytes used in the key 
were 90438104  (it uses the largest file length as the length of the key) /// I ran it AGAIN, it returned 17487ms for time, 1080 files encrypted,
bytes used in key 90438104 so it is incredibly consistent. (It prints out the stats for you at the end)  

//////////
for future versions of this code--

Yes, there are several options for improving the speed, performance, memory safety, and reliability of the code while encrypting every file and subfolder file and the file name, using only the standard libraries in Rust. Here are some suggestions:

Use a faster encryption algorithm: As mentioned earlier, using a faster and more secure encryption algorithm like AES can significantly improve the speed of the encryption process. The crypto crate in Rust provides an implementation of the AES encryption algorithm that can be used for this purpose.

Use multithreading or parallelism: The current implementation of the encrypt_files function is recursive, which can be slow for large files and directories with many nested subdirectories. One option for improving performance is to use multithreading or parallelism to encrypt multiple files simultaneously. The rayon crate in Rust provides an easy-to-use interface for parallelism that can be used to speed up the encryption process.

Optimize file I/O operations: Reading and writing files can be a bottleneck in the encryption process. Using the std::fs::File API instead of std::fs::read and std::fs::write can provide more control over the file I/O process and potentially improve performance. Additionally, using buffered I/O with the std::io::BufReader and std::io::BufWriter types can reduce the number of system calls and improve performance.

Improve error handling and reliability: The current implementation of the code uses unwrap in several places, which can cause the program to panic if there is an error. Using more robust error handling techniques, such as the Result type or the failure crate, can improve the reliability and safety of the code.

Consider security and key management: Encrypting file contents and filenames is important for securing sensitive data, but it's also important to consider key management and secure storage of encrypted data. This can include using strong key generation techniques, using different keys for different files or directories, and securely storing keys and encrypted data. The ring crate provides a comprehensive set of tools for encryption and key management that can be used for this purpose.

Overall, by incorporating these techniques and best practices, it's possible to improve the speed, performance, memory safety, and reliability of the code while encrypting every file and subfolder file and the file name, using only the standard libraries in Rust.
