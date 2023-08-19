use crate::traits::CharTrait;
use std::collections::{HashMap, HashSet};

///     Uses a key and an alphabet for transcoding substitution ciphers.
///
///     The first character of the alphabet corresponds to the first character of the
///     key, the second character of the alphabet to the second character of the key,
///     and so on. The alphabet can consist of any characters (including e.g.
///     umlauts), and the length is variable, i.e., it is not restricted to the 26 letters
///     of the alphabet.
///
///     :example:
///         ::
///
///             Alphabet: abcdefghijklmnopqrstuvwxyz
///             Key:      zebrascdfghijklmnopqtuvwxy
///
///         The letter "a" from the plaintext maps to "z" in the ciphertext, "b" to "e",
///         and so on. Thus the plaintext "flee at once. we are discovered!" is enciphered
///         as "siaa zq lkba. va zoa rfpbluaoar!"
///
///         This example was taken from
///         `Wikipedia <https://en.wikipedia.org/wiki/Substitution_cipher>`_.
///
///     #### :param str key: The key to use. Must have the same length than the alphabet.
///         It is case insensitive.
///     #### :param str alphabet: The set of characters which define the alphabet.
///         Characters which are not in the alphabet will be ignored when transcoding.
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

    ///         Checks an alphabet for consistency
    ///
    ///         Checks, if each character is unique.
    ///         
    ///         ### :param str alphabet: the alphabet to check \
    ///         ### :return: the alphabet in normalized form (i.e., in lower cases)\
    ///         ### :rtype: str
    fn check_alphabet(alphabet: &str) -> Result<String, String> {
        let alphabet = alphabet.to_lowercase();
        let alphabet_chars = alphabet.chars();

        if alphabet_chars.clone().count() != alphabet_chars.collect::<HashSet<_>>().len() {
            return Err("Alphabet characters must be unique.".to_string());
        }

        Ok(alphabet)
    }

    ///         Checks a key for consistency against a given alphabet
    ///
    ///         It is assumed that the given alphabet has already been check for consistency
    ///         before. The following checks are performed:
    ///
    ///         - the characters in the key must be unique
    ///         - the key must have the same length than the alphabet
    ///         - the set of characters in the key must be the same than the set of characters
    ///           in the alphabet
    ///
    ///         ### :param str key: the key to be validated
    ///         ### :param str alphabet: the alphabet against which the key is validated
    ///         ### :return: the validated key in normalized form (i.e., in lower cases)
    ///         ### :rtype: str
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

    ///Converts a string to upper case in a safe way
    ///<br/><br/>
    ///Reason for this function is the German "ß". \
    ///Problem: "ß".upper() results in "SS" which corrupts the xcoding translation \
    ///table. Therefore in such a case the character is simply taken as it is and \
    ///is not converted.
    ///<br/>
    ///### :Example:
    ///        "Viele Grüße".upper() results in "VIELE GRÜSSE"
    ///        _upper("Viele Grüße") results in "VIELE GRÜßE"
    ///
    ///    :param str string: the string to be converted to upper case
    ///    :return: the string converted to upper case
    ///    :rtype: str
    fn _upper(string: &str) -> String {
        string.chars().map(|x| x.to_upper_case()).collect()
    }

    /// Encodes a plaintext with the given key into the ciphertext
    ///
    ///         :param str plaintext: the plaintext to encode with the given key
    ///         :return: the resulting ciphertext
    ///         :rtype: str
    pub fn encode(&mut self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|x| *self.encode_map.get(&x).unwrap_or(&x))
            .collect()
    }

    /// Decodes a ciphertext with the given key into the plaintext
    ///
    ///         :param str ciphertext: the ciphertext to decode with the given key
    ///         :return: the resulting plaintext
    ///         :rtype: str
    pub fn decode(&mut self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|x| *self.decode_map.get(&x).unwrap_or(&x))
            .collect()
    }
}
