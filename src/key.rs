use crate::traits::CharTrait;
use std::collections::{HashMap, HashSet};

pub struct Key {
    pub key: String,
    pub alphabet: String,
    encode_map: HashMap<char, char>,
    decode_map: HashMap<char, char>,
}

impl Key {
    pub fn new(key: &str, alphabet: &str) -> Result<Self, String> {
        let alphabet = Self::check_alphabet(alphabet)?;
        let key = Self::check_key(key, &alphabet)?;

        let camel_key = Self::_upper(&key) + &key.to_lowercase();
        let camel_alphabet = Self::_upper(&alphabet) + &alphabet.to_lowercase();

        Ok(Self {
            key: key.to_lowercase(),
            alphabet,
            encode_map: camel_alphabet.chars().zip(camel_key.chars()).collect(),
            decode_map: camel_key.chars().zip(camel_alphabet.chars()).collect(),
        })
    }

    fn check_alphabet(alphabet: &str) -> Result<String, String> {
        let alphabet = alphabet.to_lowercase();
        let alphabet_chars = alphabet.chars();

        if alphabet_chars.clone().count() != alphabet_chars.collect::<HashSet<_>>().len() {
            return Err("Alphabet characters must be unique.".to_string());
        }

        Ok(alphabet)
    }

    fn check_key(key: &str, alphabet: &str) -> Result<String, String> {
        let key = key.to_lowercase();
        let key_chars = key.chars();
        let key_len = key_chars.clone().count();
        let key_uniq: HashSet<char> = key_chars.collect();

        let alphabet_chars = alphabet.chars();
        let alphabet_len = alphabet_chars.clone().count();
        let alphabet_uniq: HashSet<char> = alphabet_chars.collect();

        if key_len != key_uniq.len() {
            return Err("Key characters must be unique.".to_string());
        }

        if key_len != alphabet_len {
            return Err("Key must be as long as the alphabet.".to_string());
        }

        if key_uniq != alphabet_uniq {
            return Err("Key must use the same set of characters than the alphabet.".to_string());
        }

        Ok(key)
    }

    fn _upper(string: &str) -> String {
        string.chars().map(|x| x.to_upper_case()).collect()
    }

    pub fn encode(&mut self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|x| *self.encode_map.get(&x).unwrap_or(&x))
            .collect()
    }

    pub fn decode(&mut self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|x| *self.decode_map.get(&x).unwrap_or(&x))
            .collect()
    }
}
