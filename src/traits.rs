pub trait CharTrait {
    fn to_lower_case(&self) -> Self;
    fn to_upper_case(&self) -> Self;
}

impl CharTrait for char {
    fn to_lower_case(&self) -> char {
        match self {
            'A'..='Z' => self.to_ascii_lowercase(),
            'А'..='Я' | 'Ё' => self.to_uppercase().next().unwrap(),
            _ => *self,
        }
    }

    fn to_upper_case(&self) -> char {
        match self {
            'a'..='z' => self.to_ascii_uppercase(),
            'а'..='я' | 'ё' => self.to_uppercase().next().unwrap(),
            _ => *self,
        }
    }
}