use std::env;
use std::fs;
use std::path::Path;
use std::io;
use regex::Regex;

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

        let re = Regex::new(r"([0-9A-Fa-f]{2}):[ \t]*([0-9A-Fa-f]{4}).*").unwrap();
        for line in toy.lines() {
            if let Some(caps) = re.captures(line) {
                let addr = from_hex(&caps[1]) as usize;
                let inst = from_hex(&caps[2]);
                mem[addr] = inst;
            }
        }
    }

    println!("Before");
    print_contents(&r, &mem);

    loop {
        let inst = mem[pc as usize];

        pc = pc.wrapping_add(1);

        let op = (inst >> 12) & 15;
        let d = ((inst >> 8) & 15) as usize;
        let s = ((inst >> 4) & 15) as usize;
        let t = (inst & 15) as usize;
        let addr = (inst & 255) as usize;

        if (addr == 255 && op == 8) || (r[t] == 255 && op == 10) {
            let mut input = String::new();
            println!("In:");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read stdin");
            mem[255] = from_hex(&input.trim());
        }

        match op {
            0 => break,
            1 => r[d] = r[s].wrapping_add(r[t]),
            2 => r[d] = r[s].wrapping_sub(r[t]),
            3 => r[d] = r[s] & r[t],
            4 => r[d] = r[s] ^ r[t],
            5 => r[d] = r[s].wrapping_shl(r[t] as u32),
            6 => r[d] = r[s].wrapping_shr(r[t] as u32),
            7 => r[d] = addr as i16,
            8 => r[d] = mem[addr],
            9 => mem[addr] = r[d],
            10 => r[d] = mem[(r[t] & 255) as usize],
            11 => mem[(r[t] & 255) as usize] = r[d],
            12 => if r[d] == 0 {
                    pc = addr as u8;
                },
            13 => if r[d] > 0 {
                pc = addr as u8;
            },
            14 => pc = r[d] as u8,
            15 => {
                r[d] = pc as i16;
                pc = addr as u8;
            }
            _ => unreachable!()
        }

        if (addr == 255 && op == 9) || (r[t] == 255 && op == 11) {
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
