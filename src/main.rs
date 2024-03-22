// import modules
use std::env;
use std::io::{self, Read, Write};
mod load;
mod bitpack;
mod memory;
use memory::memory::Memory;

fn main() {
    // initialize memory storage
    let mut memory = Memory::new();
    // read in program
    let program = env::args().nth(1);
    // load instructions into a vector using the load module
    let instructions = load::load(program.as_deref());
    // initialize the initial map segment at 0 using the instructions
    memory.map_segment(instructions);

    // initialize array to store registers and program counter
    let mut reg_array = [0u32; 8];
    let mut program_counter: usize = 0;

    //loop though instructions using the program counter
    loop {
        // load the instruction into memory
        let instruction: u32 = memory.load_word(0, program_counter);

        // extract the opcode and registers A, B, and C using the bitpack module
        let reg_a = bitpack::getu(instruction as u64, 3, 6) as usize;
        let reg_b = bitpack::getu(instruction as u64, 3, 3) as usize;
        let reg_c = bitpack::getu(instruction as u64, 3, 0) as usize;
        let opcode = bitpack::getu(instruction as u64, 4, 28) as usize;
    
        // use match to determine what to do with each instruction
        match opcode {
            // Conditional Move
            0 => {
                if reg_array[reg_c] != 0 {
                    reg_array[reg_a] = reg_array[reg_b];
                }
                program_counter += 1;
            }
            // Segmented Load
            1 => {
                // load word from memory with a segment key of m[reg_b] and an offset of m[reg_c]
                reg_array[reg_a] = memory.load_word(reg_array[reg_b], reg_array[reg_c] as usize);
                program_counter += 1;
            }
            // Segmented Store
            2 => {
                // store word into memory with a segment key of m[reg_a], an offset of m[reg_b], and a value of m[reg_c]
                memory.store_word(reg_array[reg_a], reg_array[reg_b] as usize, reg_array[reg_c]);
                program_counter += 1;
            }
            // Addition
            3 => {
                reg_array[reg_a] = reg_array[reg_b].wrapping_add(reg_array[reg_c]);
                program_counter += 1;
            }
            // Multiplication
            4 => {
                reg_array[reg_a] = reg_array[reg_b].wrapping_mul(reg_array[reg_c]);
                program_counter += 1;
            }
            // Division
            5 => {
                // cannot divide by 0
                if reg_array[reg_c] == 0 {
                    panic!();
                }
                else {
                    reg_array[reg_a] = reg_array[reg_b] / reg_array[reg_c];
                }
                program_counter += 1;
            }
            // Bitwise NAND
            6 => {
                reg_array[reg_a] = !(reg_array[reg_b] & reg_array[reg_c]);
                program_counter += 1;
            }
            // Halt
            7 => {
                std::process::exit(0);
            }
            // Map Segment
            8 => {
                // map segment with segment key 0 and vec of words
                reg_array[reg_b] = memory.map_segment(vec![0; reg_array[reg_c] as usize]);
                program_counter += 1;
            }
            // Unmap Segment
            9 => {
                // unmap segment with m[reg_c]
                memory.unmap_segment(reg_array[reg_c]);
                program_counter += 1;
            }
            // Output
            10 => {
                // output must only output values between 0 and 255
                if reg_array[reg_c] <= 255 {
                    let _ = io::stdout().write_all(&[reg_array[reg_c] as u8]);
                } else {
                    panic!();
                }
                program_counter += 1;
            }
            // Input
            11 => {
                // match each byte
                match io::stdin().bytes().next() {
                    // push into register C if byte is valid
                    Some(Ok(byte)) => reg_array[reg_c] = byte as u32,
                    // fail if byte is invalid
                    Some(Err(_)) => panic!(),
                    // if none, push the max u32 into register C
                    None => reg_array[reg_c] = u32::MAX,
                }
                program_counter += 1;
            }
            // Load Program
            12 => {
                // load program with source segment key m[reg_b], destination segment key (initialized as 0), offset m[reg_c], and program counter
                memory.load_program(reg_array[reg_b], 0, reg_array[reg_c] as usize, &mut program_counter);
                // don't incriment counter because load program initializes it
            }
            // Load Value
            13 => {
                // re-extract register A as the 3 bits immediately less significant than the opcode
                let new_reg_a = bitpack::getu(instruction as u64, 3, 25) as usize;
                // extract the remaining 25 bits to get the value
                let value = bitpack::getu(instruction as u64, 25, 0) as u32;
                // load the value into register A
                reg_array[new_reg_a] = value;
                program_counter += 1;
            } 
            // fail if invalid opcode
            _ => {
                panic!();
            }
        }
    }
}