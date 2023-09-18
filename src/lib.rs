use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VernamError {
    #[error("Key Too Short, MESSAGE was length '{0}' and KEY was length '{1}'")]
    KeyTooShort(String, String),
}

pub fn generate_key(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let key: String = (0..len)
        .map(|_| rng.gen_range(32..=126) as u8 as char)
        .collect();
    key
}

pub fn vernam(data: &str, key: &str) -> Result<String, VernamError> {
    if key.len() < data.len() {
        return Err(VernamError::KeyTooShort(
            data.len().to_string(),
            key.len().to_string(),
        ));
    }

    let mut cipher_text = String::new();

    for (i, data_char) in data.chars().enumerate() {
        let cipher_char = ((data_char as u32) ^ (key.chars().nth(i).unwrap() as u32)) as u8 as char;
        cipher_text.push(cipher_char);
    }

    for i in data.len()..key.len() {
        cipher_text.push(key.chars().nth(i).unwrap());
    }

    Ok(cipher_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let len = 10;
        let key = generate_key(len);
        println!("{}", key);
        assert_eq!(len, key.len());
    }

    #[test]
    fn test_vernam() {
        let message = "Test Message";
        let key = "this_is_a_very_long_key".to_string();

        let encrypted = vernam(&message, &key).unwrap();
        let decrypted = vernam(&encrypted, &key).unwrap();

        // Compare only the relevant part of the decrypted message with the original message
        assert_eq!(&decrypted[0..message.len()], message);
    }
}
