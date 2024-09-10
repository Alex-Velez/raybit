use crate::{clean, error::BitBitError, flags::Flags};

#[derive(PartialEq, Clone, Copy)]
pub enum OpCode {
    /// Brainfuck operations
    ShiftLeft,
    ShiftRight,
    Increment,
    Decrement,
    Output,
    Input,
    FlipUp,
    FlipDown,
    FlipLeft,
    FlipRight,
    // Brainfuck loops
    LoopBegin,
    LoopEnd,
    // BitBit debug symbols (?#$!)
    OutputPtr,
    OutputMemX,
    OutputMemY,
    Halt,
    // Brainfuck common patterns
    SetPtrZero,
}

impl OpCode {
    fn from(value: &[char]) -> Option<OpCode> {
        match value {
            ['[', '-', ']'] | ['[', '+', ']'] => Some(OpCode::SetPtrZero),
            ['-', '+'] => Some(OpCode::FlipDown),
            ['+', '-'] => Some(OpCode::FlipUp),
            ['<', '>'] => Some(OpCode::FlipLeft),
            ['>', '<'] => Some(OpCode::FlipRight),
            ['<'] => Some(OpCode::ShiftLeft),
            ['>'] => Some(OpCode::ShiftRight),
            ['+'] => Some(OpCode::Increment),
            ['-'] => Some(OpCode::Decrement),
            [','] => Some(OpCode::Input),
            ['.'] => Some(OpCode::Output),
            ['?'] => Some(OpCode::OutputPtr),
            ['#'] => Some(OpCode::OutputMemX),
            ['$'] => Some(OpCode::OutputMemY),
            ['!'] => Some(OpCode::Halt),
            ['['] => Some(OpCode::LoopBegin),
            [']'] => Some(OpCode::LoopEnd),
            _ => None,
        }
    }
}

/// Generate sequence of un-optimized opcodes from a source file
pub fn lex(flags: &Flags) -> Result<Vec<OpCode>, BitBitError> {
    let clean_source: Vec<char> = clean::source(flags.file_path(), flags.raw())?;
    let mut operations: Vec<OpCode> = Vec::new();
    let mut src_index = 0;
    // let mut pat_len: usize;
    // let mut end_idx: usize;

    'lex: while src_index < clean_source.len() {
        if search_pattern(3, &mut src_index, &clean_source, &mut operations)
            || search_pattern(2, &mut src_index, &clean_source, &mut operations)
            || search_pattern(1, &mut src_index, &clean_source, &mut operations)
        {
            continue 'lex;
        }
        // pat_len = 3;
        // end_idx = (src_index + pat_len).min(clean_source.len());
        // let maybe_pattern = OpCode::from(&clean_source[src_index..end_idx]);
        // if let Some(pattern) = maybe_pattern {
        //     operations.push(pattern);
        //     src_index += pat_len;
        //     continue 'lex;
        // }
        // pat_len = 2;
        // end_idx = (src_index + pat_len).min(clean_source.len());
        // let maybe_pattern = OpCode::from(&clean_source[src_index..end_idx]);
        // if let Some(pattern) = maybe_pattern {
        //     operations.push(pattern);
        //     src_index += pat_len;
        //     continue 'lex;
        // }
        // pat_len = 1;
        // end_idx = (src_index + pat_len).min(clean_source.len());
        // let maybe_pattern = OpCode::from(&clean_source[src_index..end_idx]);
        // if let Some(pattern) = maybe_pattern {
        //     operations.push(pattern);
        //     src_index += pat_len;
        //     continue 'lex;
        // }
        src_index += 1;
    }

    Ok(operations)
}

/// Search `Vec<char>` for a defined pattern of a given size.
/// Returns true if pattern found to signal for a continue.
fn search_pattern(
    pat_len: usize,
    src_index: &mut usize,
    clean_source: &Vec<char>,
    operations: &mut Vec<OpCode>,
) -> bool {
    let end_idx = (*src_index + pat_len).min(clean_source.len());
    let maybe_pattern = OpCode::from(&clean_source[*src_index..end_idx]);
    if let Some(pattern) = maybe_pattern {
        operations.push(pattern);
        *src_index += pat_len;
        // signal continue
        return true;
    }
    // no continue signal
    return false;
}
