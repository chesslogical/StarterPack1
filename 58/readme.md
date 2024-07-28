## sha3 

1- SHA3-224: 
2- SHA3-256:
3- SHA3-384: 
4- SHA3-512:  

look at how small main.rs is. that is awesome. anyways, just run sha in terminal and it shows the above menu.
you enter 4 and type "adelia" and it writes the SHA3-512 hash value to a.txt and exits. If you run it again it appends the hash value to the file. Therefore, you could make a batch fole or ssh script and have it run 
sha 1 adelia
sha 2 adelia
sha 3 adelia
sha 4 adelia
and it would show a single string of all the inputs. 

This tiny rust scrit is amazingly powerful. You can use it as a password manager, a key maker. You could enter "2 bank" for example and the resulting SHA3-256 hash return for bank could be a password. This app is extrelemly versatile and always returns the same hash value for a given input. In fact, there are many more hash value returns than there are atoms in the known universe. With a little imagination, this tiny script is god code! 


