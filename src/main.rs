use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let mut pc: u8 = 0x10;
    let mut r: [i16; 16] = [0x0000; 16];
    let mut mem: [i16; 256] = [0x0000; 256];

    {
        let args: Vec<String> = env::args().collect();

        if args.len() == 1 {
            println!("No path to .toy file provided");
            return;
        }

        let filename = &args[1];
        let file = Path::new(filename);

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

        let toy = fs::read_to_string(filename).expect("Something went wrong when reading the file");

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

            mem[addr] = inst;
        }
    }

    println!("Before");
    print_contents(&r, &mem);

    loop {
        let inst = mem[pc as usize];

        pc = pc.wrapping_add(1);

        let op = OpCode::new((inst >> 12) & 15);
        let d = ((inst >> 8) & 15) as usize;
        let s = ((inst >> 4) & 15) as usize;
        let t = (inst & 15) as usize;
        let addr = (inst & 255) as usize;

        if (addr == 255 && op == OpCode::Load) || (r[t] == 255 && op == OpCode::LoadIndirect) {
            let mut input = String::new();
            println!("In:");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read stdin");
            mem[255] = from_hex(&input.trim());
        }

        match op {
            OpCode::Halt => break,
            OpCode::Add => r[d] = r[s].wrapping_add(r[t]),
            OpCode::Subtract => r[d] = r[s].wrapping_sub(r[t]),
            OpCode::And => r[d] = r[s] & r[t],
            OpCode::Xor => r[d] = r[s] ^ r[t],
            OpCode::LeftShift => r[d] = r[s].wrapping_shl(r[t] as u32),
            OpCode::RightShift => r[d] = r[s].wrapping_shr(r[t] as u32),
            OpCode::LoadAddress => r[d] = addr as i16,
            OpCode::Load => r[d] = mem[addr],
            OpCode::Store => mem[addr] = r[d],
            OpCode::LoadIndirect => r[d] = mem[(r[t] & 255) as usize],
            OpCode::StoreIndirect => mem[(r[t] & 255) as usize] = r[d],
            OpCode::BranchZero => if r[d] == 0 {
                    pc = addr as u8;
                },
            OpCode::BranchPositive => if r[d] > 0 {
                pc = addr as u8;
            },
            OpCode::JumpRegister => pc = r[d] as u8,
            OpCode::JumpAndLink => {
                r[d] = pc as i16;
                pc = addr as u8;
            }
        }

        if (addr == 255 && op == OpCode::Store) || (r[t] == 255 && op == OpCode::StoreIndirect) {
            println!("Out:");
            println!("{0:#06X} ({0})", mem[255]);
        }

        r[0] = 0;
    }

    println!("After");
    print_contents(&r, &mem);
}

fn print_contents(r: &[i16], mem: &[i16]) {
    println!("Registers");
    print_array(&r);
    println!("Memory");
    print_array(&mem);
}

fn print_array(array: &[i16]) {
    for i in 0..array.len() {
        print!("{:#06X} ", array[i]);
        if i % 8 == 7 {
            println!("");
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

#[derive(PartialEq)]
enum OpCode {
    Halt,
    Add,
    Subtract,
    And,
    Xor,
    LeftShift,
    RightShift,
    LoadAddress,
    Load,
    Store,
    LoadIndirect,
    StoreIndirect,
    BranchZero,
    BranchPositive,
    JumpRegister,
    JumpAndLink,
}

impl OpCode {
    fn new(op: i16) -> Self {
        match op {
            0 => Self::Halt,
            1 => Self::Add,
            2 => Self::Subtract,
            3 => Self::And,
            4 => Self::Xor,
            5 => Self::LeftShift,
            6 => Self::RightShift,
            7 => Self::LoadAddress,
            8 => Self::Load,
            9 => Self::Store,
            10 => Self::LoadIndirect,
            11 => Self::StoreIndirect,
            12 => Self::BranchZero,
            13 => Self::BranchPositive,
            14 => Self::JumpRegister,
            15 => Self::JumpAndLink,
            _ => unreachable!()
        }
    }
}
