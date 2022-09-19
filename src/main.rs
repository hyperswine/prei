use clap::{Parser, Subcommand};
use std::{
    fmt::{self, Display},
    io::stdin,
    process,
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
    Init,
}

const PREI_VERSION: f32 = 0.1;

// basically, when you run prei new <name> you run this fn
fn generate_proj(dir_path: &str) {
    // should prob get version from the .version?
    let _str = format!(
        "std = {{ version = {PREI_VERSION} }}
    test = {{ version = {PREI_VERSION} }}"
    );

    // maybe in core lib you could do that
    // in std also provide an overload for easier stuff

    // create dir
    process::Command::new("mkdir")
        .arg("-p")
        .arg(dir_path)
        .output()
        .expect(&format!("Couldn't create directory at {dir_path}"));

    // copy template
    process::Command::new("cp")
        .arg("-R")
        .arg("template")
        .arg(dir_path)
        .output()
        .expect(&format!("Couldn't create directory at {dir_path}"));

    // replace text in build.rei & project.rei
    let build_rei = dir_path.to_owned() + "/build.rei";
    let project_rei = dir_path.to_owned() + "/project.rei";

    process::Command::new("sed")
        .arg("-i")
        .arg(format!("s/$require/{_str}/g"))
        .arg(build_rei)
        .output()
        .expect("Oops, couldnt replace the text in build.rei");

    process::Command::new("sed")
        .arg("-i")
        .arg("s/$version/0.1/g")
        .arg(project_rei)
        .output()
        .unwrap();
}

fn new_proj_ui() {
    // get all the vars of project.rei
    println!("Project name: ");
    let mut _str = String::new();
    stdin().read_line(&mut _str).ok().unwrap();

    println!("Author: ");
    stdin().read_line(&mut _str).ok().unwrap();

    println!("Description: ");
    stdin().read_line(&mut _str).ok().unwrap();
}

fn main() {
    let args = Args::parse();

    // if New name is defined, call the fn
    // if no name, call the ui? or ehh
    match args.command {
        Commands::New { name } => generate_proj(&name.unwrap()),
        Commands::Init => generate_proj("."),
    }
}

#[test]
fn test_ui() {
    new_proj_ui();
}
