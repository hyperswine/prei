use clap::Parser;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Target {
    NeutronRiscv64,
    NeutronAArch64,
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "neutron-riscv64" {
            return Ok(Target::NeutronRiscv64);
        } else if s == "neutron-aarch64" {
            return Ok(Target::NeutronAArch64);
        } else {
            return Err(format!("Invalid Target \"{s}\""));
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    name: String,

    #[clap(short, long, value_parser, default_value = "neutron-riscv64")]
    target: String,

    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
