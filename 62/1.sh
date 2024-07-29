#!/bin/bash

# Simple XOR encryption/decryption script for binary and text files

# Check if a file was provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file>"
    exit 1
fi

file=$1

# Check if the file exists
if [ ! -f "$file" ]; then
    echo "Error: File does not exist."
    exit 1
fi

# Hardcoded key (change this to your own key)
key="my_super_secret_key"
klen=${#key}

# Function to perform XOR
function xor() {
    local buf
    while IFS= read -r -d '' -n 1 char; do
        buf=$(printf "\\x%02x" "'$char'")
        j=$((i % klen))  # Wrap around the key index
        printf "\\x%02x" "$(( ${buf} ^ ${key:j:1} ))"
        ((i++))
    done
}

# Read the file, pass it through XOR function, and overwrite the file
i=0
xor < "$file" > "$file".tmp && mv "$file".tmp "$file"

echo "Operation completed on $file"

######
# Reliability: Reading one byte at a time is highly reliable because it 
# minimizes the chance of corrupting data due to partial reads. It
# ensures that every byte is processed exactly once.
# Performance: This method is generally slow because each read
# operation incurs overhead from the system call. For files of 
# substantial size, this can significantly slow performance.