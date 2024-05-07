Random SHA-512 Key Generator

The Random SHA-512 Key Generator is a Rust console application that generates random SHA-512 keys and saves them in individual files within a directory named "key." It provides timing information for the key generation process, reports the number of files created, and waits for user input to exit.

Prerequisites

Before running this application, ensure that you have Rust and Cargo installed on your system. 

Usage

To use the application, follow these steps:

Clone or download this repository to your local machine.

Open your terminal or command prompt.

Navigate to the project directory.

Build and run the application using Cargo:


cargo run




The application will perform the following actions:

Check if the "key" directory already exists in the current working directory.

If the "key" directory exists:

A warning message is displayed.
Press Enter to exit the application.
If the "key" directory does not exist:

The "key" directory is created.

1000 random SHA-512 keys are generated and stored in individual files inside the "key" directory.

The application measures the time taken for this operation and displays it.

The number of files created is reported.

Press Enter to exit the application.

Press Enter to exit the application when prompted.

Example

Here's an example of what the application output might look like:

vbnet
Copy code
Operation took: 2.731432ms
1000 files were written.
Press Enter to exit...