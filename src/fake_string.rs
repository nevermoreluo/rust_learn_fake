use rand::Rng;

pub struct FakeString;

const BINARY_CHAR: &'static str = "01";
const OCTAL_CHAR: &str = "01234567";
const NUMERIC_CHAR: &str = "0123456789";
// const UPPER_ALPHA_CHAR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const LOWER_ALPHA_CHAR: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHA_CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHA_NUMERIC_CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SYMBOL_CHAR: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const ALPHA_NUMERIC_SYMBOL_CHAR: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const HEXADECIMAL_CHAR : &str = "0123456789abcdefABCDEF";
const UUID_CHAR: &str = "0123456789abcdef";


impl FakeString {

    pub fn new() -> Self {
        Self{}
    }

    fn from_characters(characters: &str, length: usize) -> String {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = characters.chars().collect();
        let mut result = String::new();

        for _ in 0..length {
            let random_char = rng.gen_range(0..chars.len());
            result.push(chars[random_char]);
        }

        result
    }

    pub fn alpha(&self, length: usize) -> String {
        FakeString::from_characters(ALPHA_CHAR, length)
    }

    pub fn alphanumeric(&self, length: usize) -> String {
        FakeString::from_characters(ALPHA_NUMERIC_CHAR, length)
    }

    pub fn binary(&self, length: usize) -> String {
        FakeString::from_characters(BINARY_CHAR, length)
    }

    pub fn octal(&self, length: usize) -> String {
        FakeString::from_characters(OCTAL_CHAR, length)
    }

    pub fn hexadecimal(&self, length: usize) -> String {
        FakeString::from_characters(HEXADECIMAL_CHAR, length)
    }

    pub fn numeric(&self, length: usize) -> String {
        FakeString::from_characters(NUMERIC_CHAR, length)
    }

    pub fn sample(&self, length: usize) -> String {
        FakeString::from_characters(ALPHA_NUMERIC_SYMBOL_CHAR, length)
    }

    pub fn uuid(&self) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();
        let len = UUID_CHAR.len();
        for _ in 0..8 {
            let random_char = rng.gen_range(0..len);
            result.push(UUID_CHAR.chars().nth(random_char).unwrap());
        }

        result.push('-');

        for _ in 0..4 {
            let random_char = rng.gen_range(0..len);
            result.push(UUID_CHAR.chars().nth(random_char).unwrap());
        }

        result.push('-');

        for _ in 0..4 {
            let random_char = rng.gen_range(0..len);
            result.push(UUID_CHAR.chars().nth(random_char).unwrap());
        }

        result.push('-');

        for _ in 0..4 {
            let random_char = rng.gen_range(0..len);
            result.push(UUID_CHAR.chars().nth(random_char).unwrap());
        }

        result.push('-');

        for _ in 0..12 {
            let random_char = rng.gen_range(0..len);
            result.push(UUID_CHAR.chars().nth(random_char).unwrap());
        }

        result
    }

    pub fn symbol(&self, length: usize) -> String {
        FakeString::from_characters(SYMBOL_CHAR, length)
    }
}