use crate::{error::BitBitError, flags::Flags, lexer::OpCode};

#[derive(Debug)]
pub enum Instruction {
    ShiftLeft(usize),
    ShiftRight(usize),
    Increment(usize),
    Decrement(usize),
    Output(usize),
    Input(usize),
    FlipUp(usize),
    #[allow(dead_code)]
    FlipDown(usize),
    #[allow(dead_code)]
    FlipLeft(usize),
    #[allow(dead_code)]
    FlipRight(usize),
    OutputPtr(usize),
    OutputMemX(usize),
    OutputMemY(usize),
    Halt,
    SetPtrZero,
    LoopBegin,
    LoopEnd,
}

impl Instruction {
    fn from(value: (OpCode, usize)) -> Option<Instruction> {
        match value {
            (OpCode::ShiftLeft, count) => Some(Instruction::ShiftLeft(count)),
            (OpCode::ShiftRight, count) => Some(Instruction::ShiftRight(count)),
            (OpCode::Increment, count) => Some(Instruction::Increment(count)),
            (OpCode::Decrement, count) => Some(Instruction::Decrement(count)),
            (OpCode::Output, count) => Some(Instruction::Output(count)),
            (OpCode::Input, count) => Some(Instruction::Input(count)),
            (OpCode::FlipUp, count) => Some(Instruction::FlipUp(count)),
            (OpCode::FlipDown, count) => Some(Instruction::FlipDown(count)),
            (OpCode::FlipLeft, count) => Some(Instruction::FlipLeft(count)),
            (OpCode::FlipRight, count) => Some(Instruction::FlipRight(count)),
            (OpCode::OutputPtr, count) => Some(Instruction::OutputPtr(count)),
            (OpCode::OutputMemX, count) => Some(Instruction::OutputMemX(count)),
            (OpCode::OutputMemY, count) => Some(Instruction::OutputMemY(count)),
            (OpCode::Halt, _) => Some(Instruction::Halt),
            (OpCode::SetPtrZero, _) => Some(Instruction::SetPtrZero),
            (OpCode::LoopBegin, _) => Some(Instruction::LoopBegin),
            (OpCode::LoopEnd, _) => Some(Instruction::LoopEnd),
        }
    }
}

/// Convert a sequence of un-optimized opcodes into a sequence of optimized instructions
pub fn analyze(op_codes: Vec<OpCode>, flags: &Flags) -> Result<Vec<Instruction>, BitBitError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut op_index = 0;
    let mut count = 1;
    let mut spz_loop_stack = 0;

    'parse: while op_index < op_codes.len() {
        // skip embedded code comments
        if spz_loop_stack > 0 && op_index < op_codes.len() {
            match op_codes[op_index] {
                OpCode::LoopBegin => spz_loop_stack += 1,
                OpCode::LoopEnd => spz_loop_stack -= 1,
                _ => {}
            }
            op_index += 1;
            continue 'parse;
        }
        // detect embedded code comments
        else if op_codes[op_index] == OpCode::SetPtrZero
            && (op_index + 1) < op_codes.len()
            && op_codes[op_index + 1] == OpCode::LoopBegin
        {
            instructions.push(Instruction::SetPtrZero);
            spz_loop_stack += 1;
            op_index += 2; // skip past spz & lbg
            continue 'parse;
        }

        match op_codes[op_index] {
            OpCode::ShiftLeft
            | OpCode::ShiftRight
            | OpCode::Increment
            | OpCode::Decrement
            | OpCode::Output
            | OpCode::Input
            | OpCode::OutputPtr
            | OpCode::OutputMemX
            | OpCode::OutputMemY
            | OpCode::Halt
            | OpCode::SetPtrZero => handle(op_index, &op_codes, &mut count, &mut instructions)?,
            OpCode::FlipUp | OpCode::FlipDown | OpCode::FlipLeft | OpCode::FlipRight => {
                if !flags.raw() {
                    handle(op_index, &op_codes, &mut count, &mut instructions)?
                }
            }
            OpCode::LoopBegin => instructions.push(Instruction::LoopBegin),
            OpCode::LoopEnd => instructions.push(Instruction::LoopEnd),
        }
        op_index += 1;
    }

    Ok(instructions)
}

/// Handle duplicate opcodes
fn handle(
    op_index: usize,
    op_codes: &Vec<OpCode>,
    count: &mut usize,
    instructions: &mut Vec<Instruction>,
) -> Result<(), BitBitError> {
    if (op_index + 1) < op_codes.len() && (*op_codes)[op_index] == (*op_codes)[op_index + 1] {
        *count += 1;
    } else {
        if let Some(instruction) = Instruction::from(((*op_codes)[op_index], *count)) {
            instructions.push(instruction);
        } else {
            return Err(BitBitError::err("failure to convert opcode to instruction"));
        }
        *count = 1;
    }
    Ok(())
}
