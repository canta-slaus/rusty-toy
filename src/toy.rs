use super::opcode::OpCode;

use std::path::Path;
use std::fs;
use std::io;

pub struct Toy {
    pc: u8,
    registers: [i16; 16],
    memory: [i16; 256],
}

impl Toy {
    pub fn new() -> Self {
        Self {
            pc: 0x10,
            registers: [0x0000; 16],
            memory: [0x0000; 256]
        }
    }

    pub fn read_from_file(&mut self, path: &str) {
        let file = Path::new(path);

        if !file.exists() {
            println!("File doesn't exist");
            return;
        }

        if let Some(extension) = file.extension() {
            if extension != "toy" {
                println!("Not a .toy file");
                return;
            }
        } else {
            println!("Unknown file extension");
            return;
        }

        let toy = fs::read_to_string(path).expect("Something went wrong when reading the file");

        for line in toy.lines() {
            if line.len() < 7 {
                continue;
            }

            if line.chars().nth(2).unwrap() != ':' {
                continue;
            }

            let (addr, inst) = line.split_once(':').unwrap();
            let inst = inst.trim();

            if inst.len() < 4 {
                continue;
            }

            let inst = &inst[..4];

            if !addr.chars().all(is_hex_char) {
                continue;
            }

            if !inst.chars().all(is_hex_char) {
                continue;
            }

            let addr = from_hex(addr) as usize;
            let inst = from_hex(inst);

            self.memory[addr] = inst;
        }
    }

    pub fn print_registers(&self) {
        Self::print_array(&self.registers);
    }

    pub fn print_memory(&self) {
        Self::print_array(&self.memory);
    }

    fn print_array(array: &[i16]) {
        for i in 0..array.len() {
            print!("{:#06X} ", array[i]);
            if i % 8 == 7 {
                println!("");
            }
        }
    }

    pub fn simulate(&mut self) {
        loop {
            let inst = self.memory[self.pc as usize];

            self.pc = self.pc.wrapping_add(1);

            let op = OpCode::new((inst >> 12) & 15);
            let d = ((inst >> 8) & 15) as usize;
            let s = ((inst >> 4) & 15) as usize;
            let t = (inst & 15) as usize;
            let addr = (inst & 255) as usize;

            if (addr == 255 && op == OpCode::Load) || (self.registers[t] == 255 && op == OpCode::LoadIndirect) {
                let mut input = String::new();
                println!("In:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read stdin");
                self.memory[255] = from_hex(&input.trim());
            }

            match op {
                OpCode::Halt => break,
                OpCode::Add => self.registers[d] = self.registers[s].wrapping_add(self.registers[t]),
                OpCode::Subtract => self.registers[d] = self.registers[s].wrapping_sub(self.registers[t]),
                OpCode::And => self.registers[d] = self.registers[s] & self.registers[t],
                OpCode::Xor => self.registers[d] = self.registers[s] ^ self.registers[t],
                OpCode::LeftShift => self.registers[d] = self.registers[s].wrapping_shl(self.registers[t] as u32),
                OpCode::RightShift => self.registers[d] = self.registers[s].wrapping_shr(self.registers[t] as u32),
                OpCode::LoadAddress => self.registers[d] = addr as i16,
                OpCode::Load => self.registers[d] = self.memory[addr],
                OpCode::Store => self.memory[addr] = self.registers[d],
                OpCode::LoadIndirect => self.registers[d] = self.memory[(self.registers[t] & 255) as usize],
                OpCode::StoreIndirect => self.memory[(self.registers[t] & 255) as usize] = self.registers[d],
                OpCode::BranchZero => if self.registers[d] == 0 {
                        self.pc = addr as u8;
                    },
                OpCode::BranchPositive => if self.registers[d] > 0 {
                    self.pc = addr as u8;
                },
                OpCode::JumpRegister => self.pc = self.registers[d] as u8,
                OpCode::JumpAndLink => {
                    self.registers[d] = self.pc as i16;
                    self.pc = addr as u8;
                }
            }

            if (addr == 255 && op == OpCode::Store) || (self.registers[t] == 255 && op == OpCode::StoreIndirect) {
                println!("Out:");
                println!("{0:#06X} ({0})", self.memory[255]);
            }

            self.registers[0] = 0;
        }
    }
}

fn from_hex(s: &str) -> i16 {
    u16::from_str_radix(s, 16).unwrap() as i16
}

fn is_hex_char(c: char) -> bool {
    match c {
        '0'..='9'
      | 'A'..='F'
      | 'a'..='f' => true,
        _ => false
    }
}
