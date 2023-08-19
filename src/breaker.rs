use crate::{
    breaker_structs::{BreakerInfo, FileIterator},
    key::Key,
    traits::OptionToResult,
};
use std::f64::consts::E;
use std::fs::{self, File};

#[derive(serde::Deserialize)]
struct Obj {
    alphabet: String,
    nbr_quadgrams: usize,
    most_frequent_quadgram: String,
    max_fitness: f64,
    average_fitness: f64,
    quadgrams: Vec<u32>,
}

pub struct Breaker {
    alphabet: String,
    alphabet_length: usize,
    quadgrams: Vec<u32>,
    info: BreakerInfo,
    key: Option<String>,
}

impl Breaker {
    pub fn new(quadgram_path: &str) -> Result<Self, String> {
        let quadgram_file = fs::read_to_string(quadgram_path).map_err(|e| e.to_string())?;
        let quadgram_obj: Obj = serde_json::from_str(&quadgram_file).map_err(|e| e.to_string())?;

        let (
            alphabet,
            nbr_quadgrams,
            most_frequent_quadgram,
            average_fitness,
            max_fitness,
            quadgrams,
        ) = (
            quadgram_obj.alphabet,
            quadgram_obj.nbr_quadgrams,
            quadgram_obj.most_frequent_quadgram,
            quadgram_obj.average_fitness,
            quadgram_obj.max_fitness,
            quadgram_obj.quadgrams,
        );

        Ok(Self {
            alphabet: alphabet.clone(),
            alphabet_length: alphabet.chars().count(),
            quadgrams,
            info: BreakerInfo::new(
                Option::from(alphabet),
                Option::from(nbr_quadgrams),
                Option::from(most_frequent_quadgram),
                Option::from(average_fitness),
                Option::from(max_fitness),
            ),
            key: None,
        })
    }

    pub fn generate_quadgrams(
        carpus_path: &str,
        quadgrams_path: &str,
        alphabet: &str,
    ) -> Result<(), String> {
        let alphabet = Key::check_alphabet(alphabet)?;

        if alphabet.chars().count() > 32 {
            return Err("Alphabet must have less or equal than 32 characters".to_string());
        }

        let mut iterator = FileIterator::new(carpus_path, &alphabet)?;

        let mut quadgram_val = iterator.next().map_none("Some error".to_string())?;

        quadgram_val = (quadgram_val << 5) + iterator.next().map_none("Some error".to_string())?;
        quadgram_val = (quadgram_val << 5) + iterator.next().map_none("Some error".to_string())?;

        let mut quadgrams = vec![0.0; 32 * 32 * 32 * 32];

        for numerical_char in iterator {
            quadgram_val = ((quadgram_val & 0x7FFF) << 5) + numerical_char;
            quadgrams[quadgram_val] += 1.0
        }

        let quadgram_sum: f64 = quadgrams.iter().sum();
        let mut quadgram_min = 10000000.0;

        for val in quadgrams.iter() {
            if val != &0.0 {
                quadgram_min = f64::min(quadgram_min, *val)
            }
        }

        let offset = (quadgram_min / 10.0 / quadgram_sum).log(E);

        let mut norm = 0.0;

        for val in quadgrams.iter_mut() {
            if val != &0.0 {
                let prop = *val / quadgram_sum;
                let new_val = prop.log(E) - offset;
                *val = new_val;
                norm += prop * new_val;
            }
        }

        for val in quadgrams.iter_mut() {
            *val = (*val / norm * 1000.0).round();
        }

        let max_idx = quadgrams
            .iter()
            .enumerate()
            .max_by(|(_, &value0), (_, &value1)| f64::total_cmp(&value0, &value1))
            .map(|(idx, _)| idx)
            .unwrap_or_default();

        let max_val = quadgrams[max_idx];

        let mut max_chars = String::new();
        let mut index = max_idx as isize;

        for _ in 0..4 {
            max_chars = alphabet
                .chars()
                .nth((index & 0x1f) as usize)
                .expect("TODO: panic message")
                .to_string()
                + &max_chars;
            index >>= 5;
        }

        let average_fitness: f64 =
            quadgrams.iter().cloned().sum::<f64>() / alphabet.chars().count().pow(4) as f64;

        let value = serde_json::json!({
            "alphabet": alphabet,
            "nbr_quadgrams": quadgram_sum,
            "most_frequent_quadgram": max_chars,
            "max_fitness": max_val,
            "average_fitness": average_fitness,
            "quadgrams": quadgrams.to_vec(),
        });

        serde_json::to_writer(
            &File::create(quadgrams_path).map_err(|e| e.to_string())?,
            &value,
        )
        .map_err(|e| e.to_string())
    }
}
