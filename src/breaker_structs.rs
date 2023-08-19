use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use utf8_chars::BufReadCharsExt;

/// Class representing various information of the quadgrams for a given language
///
///     :ivar str alphabet: text representation of the alphabet
///     :ivar int nbr_quadgrams: the number of quadgrams considered for the generated
///         quadgram file. This value may be lower than the corpus size as only characters
///         from the alphabet are considered for this value
///     :ivar str most_frequent_quadgram: the most often occurring same sequence of four
///         characters within the corpus used to generate the quadgram file. For English,
///         the expected value here is "tion".
///     :ivar float average_fitness: the expected fitness of a text if all characters are
///         generated randomly with the same probability.
///     :ivar float max_fitness: the fitness for the most frequent quadgram
pub struct BreakerInfo {
    alphabet: String,
    nbr_quadgrams: usize,
    most_frequent_quadgram: String,
    average_fitness: f64,
    max_fitness: f64,
}

impl BreakerInfo {
    pub(crate) fn new(
        alphabet: Option<String>,
        nbr_quadgram: Option<usize>,
        most_frequent_quadgram: Option<String>,
        average_fitness: Option<f64>,
        max_fitness: Option<f64>,
    ) -> Self {
        Self {
            alphabet: alphabet.unwrap_or(String::new()),
            nbr_quadgrams: nbr_quadgram.unwrap_or(0),
            most_frequent_quadgram: most_frequent_quadgram.unwrap_or(String::new()),
            average_fitness: average_fitness.unwrap_or(0.0),
            max_fitness: max_fitness.unwrap_or(0.0),
        }
    }
}

/// Class representing the result for breaking a substitution cipher
///
///     :ivar str ciphertext: the original ciphertext
///     :ivar str plaintext: the resulting plaintext using the found key
///     :ivar str key: the best key found by the breaker
///     :ivar str alphabet: the alphabet used to break the cipher
///     :ivar float fitness: the fitness of the resulting plaintext
///     :ivar int nbr_keys: the number of keys tried by the breaker
///     :ivar nbr_rounds: the number of hill climbings performed, starting with a random key
///     :ivar float keys_per_second: the number of keys tried per second
///     :ivar float seconds: the time in seconds used to break the cipher
///
pub struct BreakerResult {
    ciphertext: String,
    plaintext: String,
    key: String,
    alphabet: String,
    fitness: f64,
    nbr_keys: i32,
    nbr_rounds: i32,
    keys_per_second: f64,
    seconds: f64,
}

impl BreakerResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ciphertext: Option<String>,
        plaintext: Option<String>,
        key: Option<String>,
        alphabet: Option<String>,
        fitness: Option<&f64>,
        nbr_keys: Option<&i32>,
        nbr_rounds: Option<&i32>,
        keys_per_second: Option<&f64>,
        seconds: Option<&f64>,
    ) -> Self {
        Self {
            ciphertext: ciphertext.unwrap_or_default(),
            plaintext: plaintext.unwrap_or_default(),
            key: key.unwrap_or_default(),
            alphabet: alphabet.unwrap_or_default(),
            fitness: *fitness.unwrap_or(&0.0),
            nbr_keys: *nbr_keys.unwrap_or(&0),
            nbr_rounds: *nbr_rounds.unwrap_or(&0),
            keys_per_second: *keys_per_second.unwrap_or(&0.0),
            seconds: *seconds.unwrap_or(&0.0),
        }
    }
}

impl Display for BreakerResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "key = {}", self.key)
    }
}

pub struct FileIterator {
    file: BufReader<File>,
    trans: HashMap<char, usize>,
}

impl FileIterator {
    pub fn new(file_path: &str, alphabet: &str) -> Result<Self, String> {
        let file = BufReader::new(File::open(file_path).map_err(|e| e.to_string())?);
        let trans = alphabet
            .to_lowercase()
            .chars()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        Ok(Self { file, trans })
    }
}

impl Iterator for FileIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.file.read_char() {
            Ok(Some(data)) => Some(self.trans.get(&data).copied().unwrap_or(1)),
            _ => None,
        }
    }
}
