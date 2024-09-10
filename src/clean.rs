use crate::error::BitBitError;
use std::{fs::File, io::Read, path::PathBuf};

/// Filter source code into correctly spaced command characters
pub fn source(file_path: &PathBuf, raw_mode: bool) -> Result<Vec<char>, BitBitError> {
    let file_contents = read_file(file_path)?;

    // case: raw brainfuck
    if raw_mode {
        return Ok(file_contents
            .chars()
            .filter(|c| "<>+-,.[]".contains(*c))
            .collect());
    }

    let mut cleaned_source: Vec<char> = Vec::new();
    let mut sl_comment = false;
    let mut ml_comment = false;
    let mut last_char = '\0';

    // filter out empty lines, comments, and invalid characters
    for ch in file_contents.chars() {
        match ch {
            ';' => sl_comment = true,
            '{' => ml_comment = true,
            '}' => ml_comment = false,
            '|' | ' ' | '\n' => {
                if ch == '\n' {
                    sl_comment = false;
                }
                if !(sl_comment | ml_comment) && last_char != '|' {
                    cleaned_source.push('|');
                    last_char = '|';
                }
            }
            '<' | '>' | '+' | '-' | ',' | '.' | '[' | ']' | '?' | '#' | '$' | '!' => {
                if !(sl_comment | ml_comment) {
                    cleaned_source.push(ch);
                    last_char = ch;
                }
            }
            _ => {}
        }
    }

    // case: empty source
    if cleaned_source.is_empty() || cleaned_source.iter().all(|c| *c == '|') {
        return Err(BitBitError::err("source file has no code"));
    }

    // case: unbalanced brackets
    if !is_loop_balanced(&cleaned_source) {
        return Err(BitBitError::hint("unbalanced brackets", "[]"));
    }

    Ok(cleaned_source)
}

/// Open and read file from file path
fn read_file(file_path: &PathBuf) -> Result<String, BitBitError> {
    let mut file_contents = String::new();

    // attempt open file
    if let Ok(mut file) = File::open(file_path) {
        // attempt read to string
        if let Err(e) = file.read_to_string(&mut file_contents) {
            return Err(BitBitError::hint("could not read file", e));
        }
    } else {
        return Err(BitBitError::hint(
            "could not open file",
            file_path.display(),
        ));
    }

    Ok(file_contents)
}

/// Check if non-empty source has balanced brackets
fn is_loop_balanced(source: &Vec<char>) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in source {
        match *ch {
            '[' => stack.push('['),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
