use morse_text_generator::TextGenerator;
fn main() {
    let mut text_generator = TextGenerator::new();
    text_generator.set_characters_array(&['A', 'B'].to_vec());
    text_generator.set_seed(1);
    text_generator.set_word_len(10);
    text_generator.set_words_count(5);
    let generated_text = text_generator.generate();
    for char in generated_text {
        println!("{}", char);
    }
}