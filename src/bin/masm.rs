use anyhow::{Result, bail, Context};
use mvm::{Instruction, Data, Program, NOP, PUSHF32, PUSHU32, PLUS, MINUS, MULT, JUMP, CMP, JE, JNE, JG, JGE, JL, JLE};
use std::env;
use std::io::Write;

fn program_to_bytes(program: &Program) -> Vec<u8> {
    let mut bytes = Vec::new();
    for instruction in &program.instructions {
        match instruction {
            Instruction::NOP => bytes.push(NOP),
            Instruction::PUSH(n) => {
                match n {
                    Data::FLOAT(n) => {
                        bytes.push(PUSHF32);
                        bytes.extend_from_slice(&n.to_ne_bytes());},
                    Data::NUMBER(n) => {
                        bytes.push(PUSHU32);
                        bytes.extend_from_slice(&n.to_ne_bytes());},
                };
            }
            Instruction::PLUS => bytes.push(PLUS),
            Instruction::MINUS => bytes.push(MINUS),
            Instruction::MULT => bytes.push(MULT),
            Instruction::CMP => bytes.push(CMP),
            Instruction::JUMP(n) => {
                bytes.push(JUMP);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JE(n) => {
                bytes.push(JE);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JNE(n) => {
                bytes.push(JNE);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JG(n) => {
                bytes.push(JG);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JGE(n) => {
                bytes.push(JGE);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JL(n) => {
                bytes.push(JL);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
            Instruction::JLE(n) => {
                bytes.push(JLE);
                bytes.extend_from_slice(&n.to_ne_bytes());
            },
        }
    }
    bytes
}



fn write(program: &Program, filename: &String) -> Result<()> {
    let mut file = std::fs::File::create(filename)?;
    let bytes = program_to_bytes(program);
    file.write_all(bytes.as_slice())?;
    Ok(())
}

fn parseasm(asm: &str) -> Result<Program> {
    let mut program = Program{ instructions : Vec::new()};
    for line in asm.lines() {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().context("no instruction")?;
        match instruction {
            "NOP" => program.instructions.push(Instruction::NOP),
            "PUSH" => {
                let data = parts.next().context("no data")?;
                let num: Data;
                if let Ok(n) = data.parse::<u32>() {
                    num = Data::NUMBER(n);
                } else if let Ok(n) = data.parse::<f32>() {
                    num = Data::FLOAT(n);
                } else {
                    bail!("not an integer of float {}", data);

                }
                program.instructions.push(Instruction::PUSH(num));
            },
            "PLUS" => program.instructions.push(Instruction::PLUS),
            "MINUS" => program.instructions.push(Instruction::MINUS),
            "MULT" => program.instructions.push(Instruction::MULT),
            "CMP" => program.instructions.push(Instruction::CMP),
            "JUMP" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JUMP(data));
            },
            "JE" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JE(data));
            },
            "JNE" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JNE(data));
            },
            "JG" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JG(data));
            },
            "JGE" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JGE(data));
            },
            "JL" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JL(data));
            },
            "JLE" => {
                let data = parts.next().context("no data")?;
                let data = data.parse::<u32>()?;
                program.instructions.push(Instruction::JLE(data));
            },
            _ => bail!("unknown instruction"),
        }
    }
    Ok(program)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: mvm <asm> <outbinary>");
        return;
    }
    let filename = args.get(1).expect("no filename given");
    let outfilename = args.get(2).expect("no output filename given");
    let asm = std::fs::read_to_string(filename).expect("couldnt read file");
    let program = parseasm(&asm).unwrap();
    write(&program, outfilename).unwrap();
}
