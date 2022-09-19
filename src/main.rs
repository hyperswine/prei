use clap::{Parser, Subcommand};
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
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, value_parser, default_value = "neutron-riscv64")]
    target: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    New {
        #[clap(value_parser)]
        name: Option<String>,
    },
    Init {
        #[clap(value_parser)]
        name: Option<String>,
    }
}

// basically, when you run prei new <name> you run this fn
fn generate_proj() {
    // if executable, go into project.rei
    // build.rei
}

fn main() {
    let args = Args::parse();
}
