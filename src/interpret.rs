use crate::{
    analysis::Instruction, bindings, error::BitBitError, flags::Flags, lexer::OpCode, memory, read,
};
use std::collections::HashMap;

pub struct Program {
    memory: Vec<u8>,
    loop_table: HashMap<usize, usize>,
    instructions: Vec<Instruction>,
    pointer: usize,
    index: usize,
    flags: Flags,
}

impl Program {
    #[inline]
    pub fn new(instructions: Vec<Instruction>, flags: Flags) -> Program {
        Program {
            memory: Vec::from([0]),
            loop_table: generate_loop_table(&instructions),
            instructions,
            pointer: 0,
            index: 0,
            flags,
        }
    }

    pub fn run(&mut self) -> Result<(), BitBitError> {
        while self.index < self.instructions.len() {
            match self.instructions[self.index] {
                Instruction::ShiftRight(amount) => {
                    memory::shift_right(&mut self.memory, &mut self.pointer, amount);
                }
                Instruction::ShiftLeft(amount) => {
                    memory::shift_left(&mut self.pointer, amount)?;
                }
                Instruction::Increment(amount) => {
                    memory::increment(&mut self.memory[self.pointer], amount);
                }
                Instruction::Decrement(amount) => {
                    memory::decrement(&mut self.memory[self.pointer], amount);
                }
                Instruction::Output(amount) => {
                    memory::output(&self.memory, &self.pointer, amount)?;
                }
                Instruction::Input(amount) => {
                    memory::input(&mut self.memory, &self.pointer, amount);
                }
                Instruction::FlipUp(amount) => {
                    if !self.flags.raw() {
                        // read function id (width of arch)
                        let id = read::usize(&self.memory, self.pointer);
                        for _ in 0..amount {
                            // pointer width cell offset
                            bindings::call(
                                id,
                                OpCode::FlipUp,
                                &mut self.memory,
                                self.pointer + memory::P_WIDTH,
                            )?;
                        }
                    }
                }
                Instruction::FlipDown(_) => unimplemented!(),
                Instruction::FlipLeft(_) => unimplemented!(),
                Instruction::FlipRight(_) => unimplemented!(),
                Instruction::SetPtrZero => memory::set_ptr_zero(&mut self.memory, &self.pointer),
                Instruction::Halt => {
                    eprintln!("BITBIT: HALT!");
                    break;
                }
                Instruction::OutputPtr(repeat) => {
                    for _ in 0..repeat {
                        eprintln!(
                            "BITBIT: {:?} : [{:?}]",
                            self.pointer, self.memory[self.pointer]
                        );
                    }
                }
                Instruction::OutputMemX(repeat) => {
                    for _ in 0..repeat {
                        eprint!("BITBIT: ");
                        for i in 0..self.memory.len() {
                            eprint!("[{:0>3?}]", self.memory[i]);
                        }
                        eprintln!();
                    }
                }
                Instruction::OutputMemY(repeat) => {
                    for _ in 0..repeat {
                        for (i, val) in self.memory.iter().enumerate() {
                            let ptr = if i == self.pointer { "<" } else { "" };
                            eprintln!("BITBIT: {:?} : [{:?}] {}", i, val, ptr);
                        }
                    }
                }
                Instruction::LoopBegin => {
                    if self.memory[self.pointer] == 0 {
                        // jump to ]
                        self.index = *self.loop_table.get(&self.index).unwrap();
                    }
                }
                Instruction::LoopEnd => {
                    if self.memory[self.pointer] != 0 {
                        // jump to [
                        self.index = *self.loop_table.get(&self.index).unwrap();
                    }
                }
            }
            self.index += 1;
        }
        Ok(())
    }
}

// Generate a hash table for loops
fn generate_loop_table(program: &Vec<Instruction>) -> HashMap<usize, usize> {
    let mut loop_table: HashMap<usize, usize> = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    for (index, instruction) in program.iter().enumerate() {
        match instruction {
            &Instruction::LoopBegin => loop_stack.push(index),
            &Instruction::LoopEnd => {
                let loop_beginning_index = loop_stack.pop().unwrap();
                loop_table.insert(loop_beginning_index, index);
                loop_table.insert(index, loop_beginning_index);
            }
            _ => {}
        }
    }
    return loop_table;
}
