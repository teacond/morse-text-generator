use rand::prelude::*;

pub struct TextGenerator {
    characters_array: Vec<char>,
    words_count: i32,
    word_len: i32,
    seed: u32,
    use_all_characters: bool,
    allow_three_chars_in_a_row: bool,
}

impl TextGenerator {
    pub fn new() -> TextGenerator {
        TextGenerator {
            characters_array: Vec::<char>::new(), 
            words_count: 5,
            word_len: 5,
            seed: 1,
            use_all_characters: true,
            allow_three_chars_in_a_row: false,
        }
    }

    pub fn generate(&mut self) -> Vec<char> {
        return gen_text(&self.characters_array, self.words_count, self.word_len, self.seed, self.use_all_characters, self.allow_three_chars_in_a_row);
    }

    pub fn set_characters_array(&mut self, characters_array: &Vec<char>) {
        self.characters_array = characters_array.to_vec();
    }

    pub fn set_words_count(&mut self, words: i32) {
        self.words_count = words;
    }

    pub fn set_word_len(&mut self, word_len: i32) {
        self.word_len = word_len;
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
    
    pub fn set_use_all_characters(&mut self, use_all_characters: bool) {
        self.use_all_characters = use_all_characters;
    }

    pub fn set_allow_three_in_a_row(&mut self, allow_three_chars_in_a_row: bool) {
        self.allow_three_chars_in_a_row = allow_three_chars_in_a_row;
    }
}

fn gen_text(characters_array: &Vec<char>, words_count: i32, word_len: i32, seed: u32, use_all_characters: bool, allow_three_chars_in_a_row: bool) -> Vec<char> {
    let mut unused_characters = Vec::<char>::new();
    unused_characters.clone_from(&characters_array);
    let mut rng = StdRng::seed_from_u64(seed.into());
    let mut new_text = Vec::<char>::new();
    let mut prev_prev_char = '~';
    let mut prev_char = '^';
    if characters_array.len() >= 2 {
        for i in 0..words_count * word_len {
            let mut new_char: char;
            if i > words_count * word_len - characters_array.len() as i32 && unused_characters.len() != 0 && use_all_characters == true { // if it is ending of text
                new_char = unused_characters.choose(&mut rng).unwrap().clone();
            }
            else {
                new_char = characters_array.choose(&mut rng).unwrap().clone();
                while new_char == prev_char && new_char == prev_prev_char && allow_three_chars_in_a_row == false { // the text must not contain 3 identical characters in a row
                    new_char = characters_array.choose(&mut rng).unwrap().clone(); // regenerate character
                }
            }
            if i % word_len == 0 && i != 0 {
                new_text.push(' ');
            }
            prev_prev_char = prev_char.clone();
            prev_char = new_char.clone();
            new_text.push(new_char); // pushing new symbol to the text
            unused_characters.retain(|&x| x != new_char); // removing character from unused_characters
        }
    }
    return new_text
}