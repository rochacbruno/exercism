/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let group_size = 5;
    let mut translated = String::new();
    for (i, ch) in atbash(plain).enumerate() {
        if i > 0 && i % group_size == 0 {
            translated.push(' ');
        } 
        translated.push(ch);
    }
    translated
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash(cipher).collect::<String>()
}

/// "Translator" gets char and returns translated char
fn translator(pos: char) -> char {
    if pos.is_ascii_alphabetic() {
        char::from(b'z' - (pos as u8 - b'a'))
    } else {
        pos
    }
}

/// "Atbash" gets the index for unique chars
fn atbash(text: &str) -> impl Iterator<Item = char> + '_ {
    // I also thought about having Map as return type ^
    text.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c|c.to_ascii_lowercase())
        .map(translator)
}
