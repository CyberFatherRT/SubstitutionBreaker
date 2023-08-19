use crate::breaker_structs::BreakerInfo;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Obj {
    alphabet: String,
    nbr_quadgrams: usize,
    most_frequent_quadgram: String,
    max_fitness: f64,
    average_fitness: f64,
    quadgrams: Vec<u32>,
}

struct Breaker {
    alphabet: String,
    alphabet_length: usize,
    quadgrams: Vec<u32>,
    info: BreakerInfo,
    key: Option<String>,
}

impl Breaker {
    fn new(quadgram_path: &str) -> Result<Self, String> {
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
}
