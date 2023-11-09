use clap::Parser;
use clap_lib_tool::{add, divide, multiply, subtract};

/// Simple calculator program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First Variable
    #[arg(long)]
    varone: i32,
    /// Second Variable
    #[arg(long)]
    vartwo: i32,
    /// Operation:  Choose Add/Subtract/Multiply/Divide
    #[arg(short, long)]
    operation: String,
}

fn main() {
    let args = Args::parse();
    //parse operation and then invoke calculator
    match args.operation.as_str() {
        "Add" => println!("{}", add(args.varone, args.vartwo)),
        "Subtract" => println!("{}", subtract(args.varone, args.vartwo)),
        "Multiply" => println!("{}", multiply(args.varone, args.vartwo)),
        "Divide" => println!("{}", divide(args.varone, args.vartwo)),
        _ => println!("Invalid Operation"),
    }
}
