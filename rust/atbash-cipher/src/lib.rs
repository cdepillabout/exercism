fn normalize(str: &str) -> String {
    str.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

fn encode_cipher_chars(str: String) -> String {
    
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // TODO: Just write this as a map?
    encode_cipher_chars(normalize(plain))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
