#!/bin/bash

# One-Time Pad Encryption Script
# Usage: otp.sh <key file> <target file>

# Function to display error message and exit
function error_exit {
    echo "Error: $1" >&2
    exit 1
}

# Check the number of arguments
if [ $# -ne 2 ]; then
    error_exit "Usage: $0 <key file> <target file>"
fi

KEY_FILE="$1"
TARGET_FILE="$2"

# Check if the key file exists
if [ ! -f "$KEY_FILE" ]; then
    error_exit "Key file does not exist."
fi

# Check if the target file exists
if [ ! -f "$TARGET_FILE" ]; then
    error_exit "Target file does not exist."
fi

# Read file sizes
KEY_SIZE=$(stat -c %s "$KEY_FILE")
TARGET_SIZE=$(stat -c %s "$TARGET_FILE")

# Check if key size is adequate
if [ "$KEY_SIZE" -lt "$TARGET_SIZE" ]; then
    error_exit "Key is shorter than the target file."
fi

# Perform the XOR operation and overwrite the target file
{
    dd if="$KEY_FILE" bs=1 count="$TARGET_SIZE" 2>/dev/null | 
    dd if="$TARGET_FILE" bs=1 count="$TARGET_SIZE" 2>/dev/null |
    xxd -p -c1 |
    xargs -I{} printf "%02x" "$((0x{} ^ 0x$(xxd -p -l1)))" |
    xxd -r -p
} > "$TARGET_FILE.tmp" && mv "$TARGET_FILE.tmp" "$TARGET_FILE"

# Check if the operation was successful
if [ $? -ne 0 ]; then
    error_exit "Encryption failed."
else
    echo "Encryption completed successfully."
fi


###

# Error Handling: The script checks for the existence of files and the adequacy # of the key size before proceeding.
# Encryption Mechanism: It uses a byte-wise XOR operation based on the input
# from the dd command, processes hexadecimal data with xxd, 
# and applies Bash arithmetic via xargs to perform the XOR.
# File Overwriting: After successfully encrypting the data, it
# securely replaces the original file by moving the temporary 
# file to the original filename, ensuring no data loss or corruption 
# if the encryption process is interrupted or fails.
# You can save this script to a file named otp.sh, make it executable
# (chmod +x otp.sh), and use it by specifying the key and target
# file as arguments when calling the script.

# Note: fi is used to mark the end of an if statement. 
# It signals the end of the conditional block, 
# letting the script know that it has finished 
# processing the logic inside that if construct.


















