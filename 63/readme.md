Key Maker CLI Application

Key Maker is a command-line application written in Rust, designed to create and analyze large key files filled with random extended ASCII characters. This application can be particularly useful for tasks requiring large amounts of random data or for testing purposes.

The Key Maker application operates in two main modes: key creation and key scanning. When executed with a size parameter (for example, key 100), the application creates a file named key.key in the current directory. The size parameter specifies the desired size of the file in megabytes (MB). For instance, key 100 generates a 100 MB file. The file is filled with random extended ASCII characters, excluding the null character (\0), ensuring a diverse range of characters. The program checks if key.key already exists. If it does, the application exits with an error to prevent overwriting existing data.

In the key scanning mode, when executed with the scan command (for example, key scan), the application reads the key.key file and analyzes its content. It counts the occurrences of each extended ASCII character in the file. The results are written to a report.txt file, listing each character and its corresponding count.

To use the application, compile it using Rust's cargo build system and run the resulting executable with the desired command. For creating a key file, use ./key <size_in_mb> (for example, ./key 100 to create a 100 MB file). To scan the key file and generate a report, use ./key scan. The application will confirm the successful creation of the key file and provide the exact file size in bytes. If scanning, it will indicate the successful generation of the report file.

By following these instructions, you can efficiently create and analyze large files filled with random extended ASCII characters using the Key Maker CLI application.


/otp

a rust app std lib only, no deps, to xor encrypt/decrypt a file. key can be from 1 byte to as long as file. chat gpt made it jul 23 2024.   after extensive testing, the two apps will be merged into one, for now each seperate app is being tested for merit. 
