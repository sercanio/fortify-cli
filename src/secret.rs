use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

pub enum SecretEncoding {
    Hex,
    Base64,
}

pub fn generate_secret(length: usize, encoding: SecretEncoding) -> String {
    let mut key = vec![0u8; length];
    rand::rng().fill_bytes(&mut key);

    match encoding {
        SecretEncoding::Hex => hex::encode(key),
        SecretEncoding::Base64 => general_purpose::STANDARD.encode(key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_hex_length() {
        let length = 16;
        let secret = generate_secret(length, SecretEncoding::Hex);
        assert_eq!(secret.len(), length * 2);
        assert!(hex::decode(&secret).is_ok());
    }

    #[test]
    fn test_secret_base64_length() {
        let length = 32;
        let secret = generate_secret(length, SecretEncoding::Base64);
        assert!(general_purpose::STANDARD.decode(&secret).is_ok());
    }
}
