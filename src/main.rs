mod error;
mod opcode;
mod toy;

use error::ToyError;
use toy::Toy;

fn main() -> Result<(), ToyError> {
    let mut toy = Toy::new();

    let args: Vec<String> = std::env::args().collect();
    toy.read_from_file(args.get(1))?;

    println!("Before");
    print_contents(&toy);

    toy.simulate();

    println!("After");
    print_contents(&toy);

    Ok(())
}

fn print_contents(toy: &Toy) {
    println!("Registers");
    toy.print_registers();
    println!("Memory");
    toy.print_memory();
}
