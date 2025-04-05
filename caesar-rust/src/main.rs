const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut key_buffer = String::new();
    println!("What code (a number between 1-26) would you like to use as key");

    std::io::stdin().read_line(&mut key_buffer).unwrap();
    let key = key_buffer.trim_end().parse::<u8>().unwrap();

    println!("What is the message you want to encrypt?");

    let mut input_buffer = String::new();
    std::io::stdin().read_line(&mut input_buffer).unwrap();
    let output: String = caeser_encode(key, &input_buffer);

    println!(
        "Encoded your message via caeser cipher with the key: {} Your encoded message:",
        key_buffer
    );

    println!("{}", output)
}

fn caeser_encode(key: u8, input: &str) -> String {
    let mut output = String::new();

    for character in input.chars() {
        let alphabet_number: Option<usize> = ALPHABET.find(character);
        match alphabet_number {
            Some(result) => {
                let new_char = if (result + key as usize) > 25 {
                    ALPHABET.as_bytes()[result + key as usize - 26]
                } else {
                    ALPHABET.as_bytes()[result + key as usize]
                };

                output.push(new_char as char)
            }
            None => output.push(character),
        }
    }
    output
}
