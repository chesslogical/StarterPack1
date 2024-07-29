#!/bin/bash
# Usage: ./otp1.sh file_to_encrypt key_file
file=$1
key=$2

# Check if both arguments are provided
if [[ -z "$file" || -z "$key" ]]; then
    echo "Usage: $0 file_to_encrypt key_file"
    exit 1
fi

# Check if the key file and the file to encrypt exist
if [[ ! -f "$file" || ! -f "$key" ]]; then
    echo "Error: File or key file does not exist."
    exit 1
fi

# Compare the sizes of the file and the key
filesize=$(stat -c%s "$file")
keysize=$(stat -c%s "$key")

# Exit if the key is shorter than the file
if [[ $keysize -lt $filesize ]]; then
    echo "Error: Key is shorter than the file to encrypt."
    exit 1
fi

# Encrypt the file and overwrite
{
    dd if="$file" bs=1 count=$filesize 2>/dev/null | 
    dd if="$key" bs=1 count=$filesize 2>/dev/null | 
    xxd -p -c256 | 
    xxd -p -r | 
    xxd -p -c256 | 
    awk '{getline k < "/dev/stdin"; for(i=1;i<=length($0);i++) printf "%02x", strtonum("0x"substr($0,i,2))^strtonum("0x"substr(k,i,2)); print ""}' - | 
    xxd -p -r > "$file"
} < "$key"

echo "Encryption completed and file overwritten."
