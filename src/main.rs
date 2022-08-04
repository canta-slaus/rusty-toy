mod toy;
mod opcode;
use toy::Toy;

fn main() {
    let mut toy = Toy::new();

    {
        let args: Vec<String> = std::env::args().collect();

        if args.len() == 1 {
            println!("No path to .toy file provided");
            return;
        }

        toy.read_from_file(&args[1]);
    }

    println!("Before");
    print_contents(&toy);

    toy.simulate();

    println!("After");
    print_contents(&toy);
}

fn print_contents(toy: &Toy) {
    println!("Registers");
    toy.print_registers();
    println!("Memory");
    toy.print_memory();
}
