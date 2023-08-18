use crate::traits::CharTrait;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub struct Key {
    key: String,
    alphabet: String,
    encode_map: HashMap<char, char>,
    decode_map: HashMap<char, char>,
}

impl Key {
    pub fn new(key: &str) -> Result<Key, String> {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
        let key_length = key.graphemes(true).count();
        if key_length != 26 {
            return Err(format!("Default alphabet have 26 character and key must have same number of character, but key have {key_length} character."));
        }

        Ok(Self {
            key: key.to_lowercase(),
            alphabet,
            encode_map: HashMap::new(),
            decode_map: HashMap::new(),
        })
    }
    pub fn new_with_alphabet(key: &str, alphabet: &str) -> Result<Key, String> {
        if key.graphemes(true).count() != alphabet.graphemes(true).count() {
            return Err("Key and alphabet must have the same length.".to_string());
        }
        Ok(Self {
            key: key.to_lowercase(),
            alphabet: alphabet.to_lowercase(),
            encode_map: HashMap::new(),
            decode_map: HashMap::new(),
        })
    }

    pub fn encode(&mut self, text: &str) -> String {
        if self.encode_map.is_empty() {
            self.encode_map = self.alphabet.chars().zip(self.key.chars()).collect();
        }

        self.ciphers(&self.encode_map, text)
    }

    pub fn decode(&mut self, text: &str) -> String {
        if self.decode_map.is_empty() {
            self.decode_map = self.key.chars().zip(self.alphabet.chars()).collect();
        }

        self.ciphers(&self.decode_map, text)
    }

    fn ciphers(&self, map: &HashMap<char, char>, text: &str) -> String {
        text.chars()
            .map(|x| {
                if x.is_lowercase() {
                    *map.get(&x).unwrap_or(&x)
                } else {
                    map.get(&x.to_lower_case()).unwrap_or(&x).to_upper_case()
                }
            })
            .collect()
    }
}
