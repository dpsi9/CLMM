use solana_program_entrypoint::entrypoint;

use crate::processor::process_instruction;

entrypoint!(process_instruction);
