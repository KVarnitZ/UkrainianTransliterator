use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, Read};
use clap::Parser;

lazy_static! {
    static ref SINGLE_CHAR_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();

        map.insert('А', "A"); map.insert('Б', "B"); map.insert('В', "V");
        map.insert('Г', "Ğ"); map.insert('Ґ', "G"); map.insert('Д', "D");
        map.insert('Е', "E"); map.insert('Ж', "Ž"); map.insert('З', "Z"); 
        map.insert('И', "Y"); map.insert('І', "I"); map.insert('Ї', "Ï");
        map.insert('К', "K"); map.insert('Л', "L"); map.insert('М', "M");
        map.insert('Н', "N"); map.insert('О', "O"); map.insert('П', "P");
        map.insert('Р', "R"); map.insert('С', "S"); map.insert('Т', "T");
        map.insert('У', "U"); map.insert('Ф', "F"); map.insert('Х', "X");
        map.insert('Ц', "C"); map.insert('Ч', "Č"); map.insert('Ш', "Š");
        map.insert('Щ', "Ŝ"); map.insert('Ь', "J"); map.insert('Й', "J");

        map.insert('а', "a"); map.insert('б', "b"); map.insert('в', "v");
        map.insert('г', "ğ"); map.insert('ґ', "g"); map.insert('д', "d");
        map.insert('е', "e"); map.insert('ж', "ž"); map.insert('з', "z");
        map.insert('и', "y"); map.insert('і', "i"); map.insert('ї', "ï");
        map.insert('к', "k"); map.insert('л', "l"); map.insert('м', "m");
        map.insert('н', "n"); map.insert('о', "o"); map.insert('п', "p");
        map.insert('р', "r"); map.insert('с', "s"); map.insert('т', "t");
        map.insert('у', "u"); map.insert('ф', "f"); map.insert('х', "x");
        map.insert('ц', "c"); map.insert('ч', "č"); map.insert('ш', "š");
        map.insert('щ', "ŝ"); map.insert('ь', "j"); map.insert('й', "j");

        map
    };
}

lazy_static! {
    static ref SEQUENCE_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
        map.insert('є', "je");
        map.insert('ю', "ju");
        map.insert('я', "ja");
        map.insert('Є', "je");
        map.insert('Ю', "ju");
        map.insert('Я', "ja");
        
        map
    };
}

lazy_static! {
    static ref REVERSE_SINGLE_MAP: HashMap<String, char> = {
        let mut map = HashMap::new();
        
        map.insert("A".to_string(), 'А'); map.insert("B".to_string(), 'Б'); 
        map.insert("V".to_string(), 'В'); map.insert("Ğ".to_string(), 'Г');
        map.insert("G".to_string(), 'Ґ'); map.insert("D".to_string(), 'Д');
        map.insert("E".to_string(), 'Е'); map.insert("Ž".to_string(), 'Ж');
        map.insert("Z".to_string(), 'З'); map.insert("Y".to_string(), 'И');
        map.insert("I".to_string(), 'І'); map.insert("Ï".to_string(), 'Ї');
        map.insert("K".to_string(), 'К'); map.insert("L".to_string(), 'Л');
        map.insert("M".to_string(), 'М'); map.insert("N".to_string(), 'Н');
        map.insert("O".to_string(), 'О'); map.insert("P".to_string(), 'П');
        map.insert("R".to_string(), 'Р'); map.insert("S".to_string(), 'С');
        map.insert("T".to_string(), 'Т'); map.insert("U".to_string(), 'У');
        map.insert("F".to_string(), 'Ф'); map.insert("X".to_string(), 'Х');
        map.insert("C".to_string(), 'Ц'); map.insert("Č".to_string(), 'Ч');
        map.insert("Š".to_string(), 'Ш'); map.insert("Ŝ".to_string(), 'Щ');
        map.insert("J".to_string(), 'Й');
        
        map.insert("a".to_string(), 'а'); map.insert("b".to_string(), 'б');
        map.insert("v".to_string(), 'в'); map.insert("ğ".to_string(), 'г');
        map.insert("g".to_string(), 'ґ'); map.insert("d".to_string(), 'д');
        map.insert("e".to_string(), 'е'); map.insert("ž".to_string(), 'ж');
        map.insert("z".to_string(), 'з'); map.insert("y".to_string(), 'и');
        map.insert("i".to_string(), 'і'); map.insert("ï".to_string(), 'ї');
        map.insert("k".to_string(), 'к'); map.insert("l".to_string(), 'л');
        map.insert("m".to_string(), 'м'); map.insert("n".to_string(), 'н');
        map.insert("o".to_string(), 'о'); map.insert("p".to_string(), 'п');
        map.insert("r".to_string(), 'р'); map.insert("s".to_string(), 'с');
        map.insert("t".to_string(), 'т'); map.insert("u".to_string(), 'у');
        map.insert("f".to_string(), 'ф'); map.insert("x".to_string(), 'х');
        map.insert("c".to_string(), 'ц'); map.insert("č".to_string(), 'ч');
        map.insert("š".to_string(), 'ш'); map.insert("ŝ".to_string(), 'щ');
        map.insert("j".to_string(), 'й');
        
        map
    };
}

lazy_static! {
    static ref REVERSE_SEQUENCE_MAP: Vec<(String, char)> = {
        vec![
            ("ja".to_string(), 'я'),
            ("Ja".to_string(), 'Я'),
            ("JA".to_string(), 'Я'),
            
            ("je".to_string(), 'є'),
            ("Je".to_string(), 'Є'),
            ("JE".to_string(), 'Є'),
            
            ("ju".to_string(), 'ю'),
            ("Ju".to_string(), 'Ю'),
            ("JU".to_string(), 'Ю'),
        ]
    };
}

#[derive(Clone)]
struct Word {
    text: String,
    chars: Vec<char>,
}

fn is_word_all_uppercase(word: &Word) -> bool {
    let letters: Vec<char> = word.chars.iter().filter(|c| c.is_alphabetic()).copied().collect();
    if letters.is_empty() {
        return false;
    }
    
    if letters.len() == 1 {
        return false;
    }
    
    let has_uppercase = letters.iter().any(|c| c.is_uppercase());
    let has_lowercase = letters.iter().any(|c| c.is_lowercase());
    
    has_uppercase && !has_lowercase
}

fn should_force_uppercase_single_char(words: &[Word], index: usize) -> bool {
    let word = &words[index];
    
    let letters: Vec<char> = word.chars.iter().filter(|c| c.is_alphabetic()).copied().collect();
    if letters.len() != 1 || !letters[0].is_uppercase() {
        return false;
    }
    
    let mut context_words = Vec::new();
    
    let mut left_count = 0;
    for i in (0..index).rev() {
        if !words[i].chars.is_empty() {
            context_words.push(&words[i]);
            left_count += 1;
            if left_count >= 2 {
                break;
            }
        }
    }
    
    let mut right_count = 0;
    for i in (index + 1)..words.len() {
        if !words[i].chars.is_empty() {
            context_words.push(&words[i]);
            right_count += 1;
            if right_count >= 3 {
                break;
            }
        }
    }
    
    if context_words.is_empty() {
        return false;
    }
    
    let mut uppercase_count = 0;
    let mut total_count = 0;
    
    for &ctx_word in &context_words {
        let ctx_letters: Vec<char> = ctx_word.chars.iter()
            .filter(|c| c.is_alphabetic())
            .copied()
            .collect();
        
        if ctx_letters.len() > 1 {
            total_count += 1;
            if is_word_all_uppercase(ctx_word) {
                uppercase_count += 1;
            }
        }
    }
    
    total_count >= 2 && uppercase_count >= 2 && uppercase_count as f32 / total_count as f32 >= 0.5
}

fn normalize_char(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

fn transliterate_char(c: char, force_upper: bool, prev_char: Option<char>) -> String {
    if let Some(replacement) = SINGLE_CHAR_MAP.get(&c) {
        (*replacement).to_string()
    } else if let Some(&base_sequence) = SEQUENCE_MAP.get(&c) {
        let is_upper = c.is_uppercase();
        
        let skip_consonant = if let Some(prev) = prev_char {
            normalize_char(prev) == normalize_char(c)
        } else {
            false
        };

        let result = if skip_consonant {
            &base_sequence[1..]
        } else {
            base_sequence
        };

        if force_upper {
            result.to_uppercase()
        } else if is_upper {
            let mut upper_sequence = String::from(result);
            if let Some(first_char) = upper_sequence.chars().next() {
                let upper_char = first_char.to_uppercase().to_string();
                upper_sequence.replace_range(..first_char.len_utf8(), &upper_char);
            }
            upper_sequence
        } else {
            result.to_string()
        }
    } else {
        c.to_string()
    }
}

fn transliterate(text: &str) -> String {
    let mut result = String::new();
    let mut current_sentence = String::new();
    
    for c in text.chars() {
        current_sentence.push(c);
        
        if matches!(c, '.' | '?' | '!' | '\n') {
            result.push_str(&transliterate_sentence(&current_sentence));
            current_sentence.clear();
        }
    }
    
    if !current_sentence.is_empty() {
        result.push_str(&transliterate_sentence(&current_sentence));
    }
    
    result
}

fn transliterate_sentence(text: &str) -> String {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut word_chars = Vec::new();

    for c in text.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
            word_chars.push(c);
        } else {
            if !current_word.is_empty() {
                words.push(Word {
                    text: current_word.clone(),
                    chars: word_chars.clone(),
                });
                current_word.clear();
                word_chars.clear();
            }
            words.push(Word {
                text: c.to_string(),
                chars: vec![],
            });
        }
    }

    if !current_word.is_empty() {
        words.push(Word {
            text: current_word,
            chars: word_chars,
        });
    }

    let mut result = String::new();
    
    for (i, word) in words.iter().enumerate() {
        if word.chars.is_empty() {
            result.push_str(&word.text);
        } else {
            let is_all_upper = is_word_all_uppercase(word);
            let force_upper = is_all_upper || should_force_uppercase_single_char(&words, i);
            
            let mut prev_char: Option<char> = None;
            for &c in &word.chars {
                result.push_str(&transliterate_char(c, force_upper, prev_char));
                prev_char = Some(c);
            }
        }
    }

    result
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_lowercase().next().unwrap_or(c), 'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ï')
}

fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

fn is_sibilant_or_hissing(c: char) -> bool {
    matches!(c.to_lowercase().next().unwrap_or(c), 'c' | 'č' | 'š' | 'ŝ' | 'ž' | 'z' | 's')
}

fn reverse_transliterate(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let mut matched = false;
        
        if i + 2 < chars.len() {
            let three = format!("{}{}{}", chars[i], chars[i+1], chars[i+2]);
            if three == "tsj" {
                result.push_str("ться");
                i += 3;
                matched = true;
            }
        }
        
        if !matched && i + 1 < chars.len() {
            let two_char: String = chars[i..=i+1].iter().collect();
            
            for (pattern, cyrillic) in REVERSE_SEQUENCE_MAP.iter() {
                if two_char == *pattern {
                    result.push(*cyrillic);
                    i += 2;
                    matched = true;
                    break;
                }
            }
            
            if !matched {
                if let Some(&cyrillic) = REVERSE_SINGLE_MAP.get(&two_char) {
                    result.push(cyrillic);
                    i += 2;
                    matched = true;
                }
            }
        }
        
    if !matched {
        let c = chars[i];
    
        if c == 'j' || c == 'J' {
            let prev_char = if i > 0 { Some(chars[i-1]) } else { None };
            
            let next_char = if i + 1 < chars.len() { Some(chars[i+1]) } else { None };
            
            let after_sibilant = prev_char.map_or(false, |p| is_sibilant_or_hissing(p));
            let before_vowel = next_char.map_or(false, |n| is_vowel(n));
            
            if after_sibilant && before_vowel {
                if c == 'J' {
                    result.push('Ь');
                } else {
                    result.push('ь');
                }
                i += 1;
                matched = true;
            } else {
                let after_consonant = prev_char.map_or(false, |p| is_consonant(p));
                let before_consonant_or_end = next_char.map_or(true, |n| is_consonant(n) || !n.is_alphabetic());
                
                if after_consonant && before_consonant_or_end {
                    if c == 'J' {
                        result.push('Ь');
                    } else {
                        result.push('ь');
                    }
                } else {
                    if c == 'J' {
                        result.push('Й');
                    } else {
                        result.push('й');
                    }
                }
                i += 1;
                matched = true;
            }
        } else {
            let one_char = c.to_string();
            if let Some(&cyrillic) = REVERSE_SINGLE_MAP.get(&one_char) {
                result.push(cyrillic);
                i += 1;
                matched = true;
            }
        }
    }
        
        if !matched {
            result.push(chars[i]);
            i += 1;
        }
    }
    
    result
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from(""))]
    text: String,
    
    #[arg(short, long, help = "Zvorotnja transliteracija (Latynycja -> Kyrylycja)")]
    reverse: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let input_text: String;

    if !args.text.is_empty() {
        input_text = args.text;
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        input_text = buffer;
    }

    let result = if args.reverse {
        reverse_transliterate(&input_text)
    } else {
        transliterate(&input_text)
    };

    print!("{}", result);

    Ok(())
}
