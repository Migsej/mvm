use anyhow::Result;
use std::env;

use mvm::{Instruction, Data, Program, Machine, NOP, PUSHU32, PUSHF32, PLUS, MINUS, MULT, JUMP, CMP, JE, JG, JNE, JGE, JL, JLE};

fn readf32(bytes: &[u8], index: &mut usize) -> Result<f32> {
    let n = f32::from_ne_bytes(bytes[*index+1..*index+5].try_into()?);
    *index += 4;
    Ok(n)
}
fn readu32(bytes: &[u8], index: &mut usize) -> Result<u32> {
    let n = u32::from_ne_bytes(bytes[*index+1..*index+5].try_into()?);
    *index += 4;
    Ok(n)
}

fn bytes_to_program(bytes: &[u8]) -> Result<Program> {
    let mut instructions = Vec::new();
    println!("{:?}", bytes);
    let mut byteindex = 0;
    while byteindex < bytes.len() {
        match bytes[byteindex] {
            NOP => {instructions.push(Instruction::NOP); },
            PUSHF32 => {
                let n = readf32(bytes, &mut byteindex)?;
                instructions.push(Instruction::PUSH(Data::FLOAT(n)));
            },
            PUSHU32 => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::PUSH(Data::NUMBER(n)));
            },
            PLUS => {instructions.push(Instruction::PLUS); },
            MINUS => {instructions.push(Instruction::MINUS); },
            MULT => {instructions.push(Instruction::MULT); },
            JUMP => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JUMP(n));
            },
            CMP => {instructions.push(Instruction::CMP); },
            JE => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JE(n));
            },
            JNE => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JNE(n));
            },
            JL => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JL(n));
            },
            JLE => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JLE(n));
            },
            JG => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JG(n));
            },
            JGE => {
                let n = readu32(bytes, &mut byteindex)?;
                instructions.push(Instruction::JGE(n));
            },


            _ => panic!("unknown instruction"),
        }
        byteindex += 1;
    }
    Ok(Program { instructions })
}


fn read(filename: &String) -> Result<Program> {
    let bytes = std::fs::read(filename)?;
    let program = bytes_to_program(&bytes)?;
    Ok(program)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: mvm <binary>");
        return;
    }

    let readprogram = read(&args[1]).unwrap();
    println!("{:?}", readprogram);
    
    let mut machine = Machine::new(readprogram);
    machine.run().unwrap();
    println!("{:?}", machine.stack);
    println!("{:#?}", machine);

}



