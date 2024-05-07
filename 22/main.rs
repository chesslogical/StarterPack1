use pbkdf2::pbkdf2;
use pbkdf2::PasswordHasher;
use crypto::symmetriccipher::SymmetricCipherError;
use crypto::aes::{KeySize, ecb_encryptor};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};

fn generate_aes256_key(password: &[u8]) -> Result<[u8; 32], SymmetricCipherError> {
    let mut key = [0u8; 32];
    let salt = [0u8; 16]; // Use a random salt for better security
    let iterations = 100_000; // Choose a high number of iterations for better security
    pbkdf2::<crypto::sha2::Sha256>(password, &salt, iterations, &mut key);
    Ok(key)
}

fn main() {
    // Example usage: generate an AES-256 key from a password
    let password = b"my secret password";
    let key = generate_aes256_key(password).unwrap();

    // Encrypt a message using the key
    let plaintext = b"hello, world!";
    let mut buffer = [0u8; 16];
    let mut encryptor = ecb_encryptor(KeySize::KeySize256, &key, NoPadding);
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&plaintext);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    let ciphertext = write_buffer.take_read_buffer().take_remaining().to_vec();

    println!("Plaintext: {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
}
