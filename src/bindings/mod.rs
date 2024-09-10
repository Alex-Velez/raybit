use crate::{error::BitBitError, lexer::OpCode};

type Memory = Vec<u8>;
type Pointer = usize;
type FnOutput = Result<Option<(Pointer, Vec<u8>)>, BitBitError>;

mod map;
mod raudio;
mod rcore;
mod rmodels;
mod rshapes;
mod rtext;
mod rtextures;

pub fn call(
    id: usize,
    flip: OpCode,
    memory: &mut Memory,
    pointer: Pointer,
) -> Result<(), BitBitError> {
    match flip {
        OpCode::FlipUp => {
            // validate function id range
            if id >= map::FN_COUNT {
                return Err(BitBitError::hint("function index out of bounds", id));
            }

            // SAFETY: trust raylib :P
            unsafe {
                // Store output of function into memory at output pointer
                if let Some(output) = map::FUNCTIONS[id](memory, pointer)? {
                    let output_ptr = output.0;
                    let output_cells = output.1;
                    for (i, value) in output_cells.iter().enumerate() {
                        (*memory)[output_ptr + i] = *value;
                    }
                }
            }
        }
        OpCode::FlipDown => unimplemented!(),
        OpCode::FlipLeft => unimplemented!(),
        OpCode::FlipRight => unimplemented!(),
        _ => {
            return Err(BitBitError::err("invalid flip code"));
        }
    }

    Ok(())
}
