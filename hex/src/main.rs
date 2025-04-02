use clap::Parser;
use num_traits::Num;
use num_bigint::BigUint;

enum InputType {
    Hex,
    Int,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    value: String,
    #[arg(short = 'x', long = "hex", help = "Force hex to integer conversion", default_value_t = false)]
    hex_to_int: bool,
    #[arg(short = 'i', long = "int", help = "Force integer to hex conversion", default_value_t = false)]
    int_to_hex: bool,
}

impl Args {
    fn is_hex(&self) -> bool {
        self.value.starts_with("0x") || self.value.starts_with("0X") || self.value.chars().all(|c| c.is_digit(16))
    }

    fn is_int(&self) -> bool {
        self.value.chars().all(|c| c.is_digit(10))
    }

    pub fn input_type(&self) -> InputType {
        if self.hex_to_int {
            if !self.is_hex() {
                eprintln!("Error: invalid hex value");
                std::process::exit(1);
            }

            return InputType::Hex;
        } else if self.int_to_hex {
            if !self.is_int() {
                eprintln!("Error: invalid int value");
                std::process::exit(1);
            }
            
            return InputType::Int;
        }

        if self.is_hex() {
            InputType::Hex
        } else if self.is_int() {
            InputType::Int
        } else {
            eprintln!("Error: invalid input value");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.hex_to_int && args.int_to_hex {
        eprintln!("Error: cannot specify both hex and int flags");
        std::process::exit(1);
    }

    match args.input_type() {
        InputType::Hex => {
            let value_str = if args.value.starts_with("0x") || args.value.starts_with("0X") {
                &args.value[2..]
            } else {
                &args.value
            };

            let value = BigUint::from_str_radix(&value_str, 16).unwrap();
            println!("{}", value.to_str_radix(10));
        }
        InputType::Int => {
            let value = BigUint::from_str_radix(&args.value, 10).unwrap();
            println!("0x{}", value.to_str_radix(16));
        }
    }
}
