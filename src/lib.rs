use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub struct Key {
    key: String,
    alphabet: String,
    map: HashMap<char, char>,
}

impl Key {
    pub fn new(key: &str) -> Result<Key, String> {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
        let key_length = key.graphemes(true).count();
        if key_length != 26 {
            return Err(format!("Default alphabet have 26 character and key must have same number of character, but key have {key_length} character."));
        }

        Ok(Self {
            key: key.to_string(),
            alphabet,
            map: HashMap::new(),
        })
    }
    pub fn new_with_alphabet(key: &str, alphabet: &str) -> Result<Key, String> {
        if key.graphemes(true).count() != alphabet.graphemes(true).count() {
            return Err("Key and alphabet must have the same length.".to_string());
        }
        Ok(Self {
            key: key.to_string(),
            alphabet: alphabet.to_string(),
            map: HashMap::new(),
        })
    }

    pub fn encode(&mut self, text: &str) -> Result<String, String> {
        if self.map.is_empty() {
            self.map = self.alphabet.chars().zip(self.key.chars()).collect();
        }

        Ok(text
            .chars()
            .map(|x| *self.map.get(&x).unwrap_or(&x))
            .collect())
    }

    pub fn decode(&self, text: &str) -> Result<String, String> {
        todo!()
    }
}

fn main() {}
