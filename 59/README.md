


There are dozens of different ways to encrypt files via rust! Here are some of the ways. Note- all apps in RustEncrypt strictly use std lib only- adding
crates from crates.io drastically enhances the options, but this repo, /RustEncrypt is only using std library, which is limited for encryption. 

a- simple otp no deps overwrites main file. not for large files, file and key are read into memory. 

b-This refactored function includes better error handling, and it processes the file contents and key together, ensuring that both are properly read before proceeding with encryption. Additionally, it avoids repeatedly opening the file for writing within the loop, which was inefficient and unnecessary.  overwrites main file. not for large files, file and key are read into memory. 

c- Writes to different file, safer. Can handle large and small files fairly well due to the use of chunk-based processing.

d- Advanced.  See the readme in the /4 directory

e- more advanced than 4. 

f-  The first password app.  file encryption via password. can handle very large files. 

g- a bit of a different take on password encryption. it is not as advanced as pass1 but still nice. 
