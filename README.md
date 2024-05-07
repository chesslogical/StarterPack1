
 This readme is a Work In Progress. Duplicates mean the higher number duplicate is intended for re-write or modification. Do NOT run any of this code unless you understand what it does. If possible you should get an old computer and use it exclusively to test/learn rust code and re-install the OS frequently. 

  Code for the discerning Rust coder. 

1- Returns Sha-3-512 hash value for any string. windows.  Readme in the directory. 

2- CLI otp app -WIP

3- AES makes another file, does not overwrite WIP 

4- Console txtcolors

5- list of rust videos-- will replace this with something better

6- Scan- scans any file, generates report that lists how many of each extended ascii chars in the file. 

7- Key maker- run 10 1000 will make 10 keys of 1000 bytes. 

8- simple encryption- no external key, no requested password. It only asks for a file name, and then processes the file.

9- This Rust script makes 10,000 files in the blink of an eye and fills each one with random binary input. It is pretty much useless- that is unless you want a rust script that makes 10,000 files and puts some random chars in each one- in which case you are in luck.

10- simple rust encryption. It asks for a file name then encrypts the file. Run it again, it decrypts the file. Extremely fast.

11- A console game in Rust

12- The Rust code generates a random 256-bit (32-byte) key for the Advanced Encryption Standard (AES) cipher.

13- This script uses the std::fs::File and std::io modules to read and write files, and prompts the user for input using the std::io::stdin() function. It reads the contents of the specified file into a Vec, encrypts or decrypts the contents using a simple XOR cipher with the specified password, and writes the encrypted or decrypted contents back to the file.

14- The code is a simple command-line application written in Rust that takes a file name and password from the user, reads the contents of the specified file, encrypts or decrypts the contents using the XOR cipher algorithm with the provided password, and writes the result back to the file. The code also uses the zeroize crate to zeroize sensitive data (such as the file name, password, and file contents) from memory after they are no longer needed.

15- The file-encrypter program is a command line utility that allows a user to encrypt or decrypt the contents of a file using a password.

16- The program is a simple command-line tool that allows the user to encrypt or decrypt the contents of a file using a key file. When the program is run, it displays a menu with three options: encrypt, decrypt, or exit. If the user selects the encrypt or decrypt option, the program prompts them to enter the name of the file they want to operate on and the name of the key file.

17- One Time Pad 

18- This script defines a constant named SHRED_DIR [./shred] that specifies the name of the directory where the files to be encrypted are located. NOTE- this is ONE WAY encryption. There is NO decryption. DO NOT put anything in the shred directory that you want to save. There are NO take backs!

19- another file shredder

20-  Yet  another file shredder

21- shreds entire directory. No way to decrypt

22- In this version, the maximum value for the input field is set to 115792089237316195423570985008687907853269984665640564039457584007913129639935 using the max attribute, which is the maximum input value that the SHA-256 hash function can handle. Therefore, this script can generate a unique password for any input number between 1 and the maximum input value that can be handled by the SHA-256 hash function, while ensuring that the same password is returned every time the same number is given.

23- The Random SHA-512 Key Generator is a Rust console application that generates random SHA-512 keys and saves them in individual files within a directory named "key".


24- Shows some Linux system info

25- This Rust program is a SHA3-512 hash calculator that takes user input, validates it, computes its SHA3-512 hash, and displays the result. 

26- Showing cpu core temp in Linux. 

27- Encrypt or decrrypt a file! No key needed, no password needed. The app does all the work for you. 


28- Simulation of Conway's Game of Life in Rust.

29- USE CAUTION. This creates a random key then encrypts an entire folder and all its subfolders-, but does not decrypt it. More of a directory shredder than an encryption tool. 

30- Encryption- read full readme or you will NOT be able to decrypt

31- Below is a complete example of a simple console-based password manager in Rust. Please remember that this is a basic example meant for educational purposes and not intended for actual use to manage your passwords, as it lacks proper security measures.

32- Rust Encryption/Decryption otp.

33- Single threaded webserver in Rust

34- txtcolor

35- Creates a new file called report.txt that lists every extended ascii character, and lists how many instances of each extended ascii character are in the file. Very reliable.

36- Keymake 1

37- This Rust script makes 10,000 files in the blink of an eye and fills each one with random binary input. It is pretty much useless- that is unless you want a rust script that makes 10,000 files and puts some random chars in each one- in which case you are in luck.

38- simple encryption no external key, no requested password. It only asks for a file name, and then processes the file. Overwrites the file, so for critical data make sure copy of file is stored somewhere. Tested on 500mb text file, seems to encrypt and decrypt fine.

39- simple rust encryption. It asks for a file name then encrypts the file. Run it again, it decrypts the file. Extremely fast.

40- Rust Encryption/Decryption otp.

41- The Rust code generates a random 256-bit (32-byte) key for the Advanced Encryption Standard (AES) cipher.

42- This script uses the std::fs::File and std::io modules to read and write files, and prompts the user for input using the std::io::stdin() function. It reads the contents of the specified file into a Vec, encrypts or decrypts the contents using a simple XOR cipher with the specified password, and writes the encrypted or decrypted contents back to the file.

43- The code is a simple command-line application written in Rust that takes a file name and password from the user, reads the contents of the specified file, encrypts or decrypts the contents using the XOR cipher algorithm with the provided password, and writes the result back to the file. The code also uses the zeroize crate to zeroize sensitive data (such as the file name, password, and file contents) from memory after they are no longer needed.

44-  The file-encrypter program is a command line utility that allows a user to encrypt or decrypt the contents of a file using a password. The program is designed with security and memory safety in mind, and it employs several best practices to achieve these goals.

45- The program is a simple command-line tool that allows the user to encrypt or decrypt the contents of a file using a key file. When the program is run, it displays a menu with three options: encrypt, decrypt, or exit. If the user selects the encrypt or decrypt option, the program prompts them to enter the name of the file they want to operate on and the name of the key file.

46- If you use this right it can do the OTP method. The script is a command-line tool that can encrypt or decrypt a file using a key file. It works by reading the input file and key file into memory, then XORing each byte of the input file with the corresponding byte of the key file to produce an encrypted output file. The original file is overwritten with the encrypted output file.

47- shredders- no decryption!!! 

48- key maker

49- This Rust program is a SHA3-512 hash calculator that takes user input, validates it, computes its SHA3-512 hash, and displays the result

50- Linux sys info/core temp 

51- misc encryption utils

52- Multithreaded webserver in Rust

53- CLI timer app. I use it on windows. It simply asks for number of minutes to count down. Updates every 10 percent of countdown. Beeps 5 times when done and resets, asking for the number of minutes to count down. 

54-

55-




