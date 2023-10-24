use anyhow::{Result, bail, Context};

pub const NOP: u8 = 0;
pub const PUSHU32: u8 = 1;
pub const PUSHF32: u8 = 2;
pub const PLUS: u8 = 3;
pub const MINUS: u8 = 4;
pub const MULT: u8 = 5;
pub const JUMP: u8 = 6;
pub const CMP: u8 = 7;
pub const JE: u8 = 8;
pub const JNE: u8 = 9;
pub const JG: u8 = 10;
pub const JGE: u8 = 11;
pub const JL: u8 = 12;
pub const JLE: u8 = 13;


#[derive(Copy, Clone, Debug)]
pub enum Instruction {
   NOP, 
   PUSH(Data),
   PLUS,
   MINUS,
   MULT,
   JUMP(u32),
   CMP,
   JE(u32),
   JNE(u32),
   JG(u32),
   JGE(u32),
   JL(u32),
   JLE(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum Data {
    NUMBER(u32),
    FLOAT(f32),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Machine { 
    program: Program,
    instructionpointer: usize,
    pub stack: Vec<Data>,
    greater: bool,
    equal: bool,
    less: bool,
}

macro_rules! op {
    ($func:ident, $op:tt) => {
        fn $func(&mut self) -> Result<()> {
            let a = self.stack.pop().context("couldnt pop element from stack")?;
            let b = self.stack.pop().context("couldnt pop element from stack")?;
            if let Data::NUMBER(a) = a  {
                if let Data::NUMBER(b) = b {
                    self.stack.push(Data::NUMBER(b $op a));
                    return Ok(());
                }
            }
            if let Data::FLOAT(a) = a  {
                if let Data::FLOAT(b) = b {
                    self.stack.push(Data::FLOAT(b $op a));
                    return Ok(());
                }
            }
            bail!("couldnt perform operation");
        }
    };
}

impl Machine {
    pub fn new(program: Program) -> Machine {
        Machine { instructionpointer: 0,
                  stack: Vec::new(),
                  program,
                  greater: false,
                  equal: false,
                  less: false,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        match &self.program.instructions[self.instructionpointer] {
            Instruction::NOP => {},
            Instruction::PUSH(n) => {self.stack.push(n.clone());},
            Instruction::PLUS => self.plus()?,
            Instruction::MINUS => self.minus()?,
            Instruction::MULT => self.mult()?,
            Instruction::JUMP(n) => {self.instructionpointer = *n as usize; return Ok(());},
            Instruction::CMP => self.cmp()?,
            Instruction::JE(n) => {if self.equal {self.instructionpointer = *n as usize; return Ok(());}},
            Instruction::JNE(n) => {if !self.equal {self.instructionpointer = *n as usize; return Ok(());}},
            Instruction::JG(n) => {if self.greater {self.instructionpointer = *n as usize; return Ok(());}},
            Instruction::JL(n) => {if self.less {self.instructionpointer = *n as usize; return Ok(());}},
            Instruction::JGE(n) => {if !self.less {self.instructionpointer = *n as usize; return Ok(());}},
            Instruction::JLE(n) => {if !self.greater {self.instructionpointer = *n as usize; return Ok(());}},

        }
        self.instructionpointer += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while self.instructionpointer < self.program.instructions.len() {
            println!("{:?}", self.program.instructions[self.instructionpointer]);
            self.step()?;
        }
        Ok(())
    }
    
    op!(mult, *);
    op!(minus, -);
    op!(plus, +);


    fn cmp(&mut self) -> Result<()> {
        let a = self.stack[self.stack.len()-1];
        let b = self.stack[self.stack.len()-2];
        if let Data::NUMBER(a) = a  {
            if let Data::NUMBER(b) = b {
                self.greater = b > a;
                self.equal = b == a;
                self.less = b < a;
                return Ok(());
            } else {
                bail!("expected integer {:?} {:?}", a, b);
            }
        }
        if let Data::FLOAT(a) = a  {
            if let Data::FLOAT(b) = b {
                self.greater = b > a;
                self.equal = b == a;
                self.less = b < a;
                return Ok(());
            } else {
                bail!("expected float {:?} {:?}", a, b);
            }
        }
        bail!("couldnt compare {:?} {:?}", a, b);
    }

}
