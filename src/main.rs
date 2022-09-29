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

    // add a -repl flag to start the repl if needed?
    // uhh hmm IDK
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
const URL: &'static str = "https://github.com/hyperswine/prei-template.git";
const PREI_ZIP: &'static str = "v1.zip";

// NOTE: prei new only works on relative paths

// basically, when you run prei new <name> you run this fn
fn generate_proj(dir_path: &str, mv: bool) {
    let _str = format!(
        "std = {{ version = {PREI_VERSION} }}
    test = {{ version = {PREI_VERSION} }}"
    );

    // fetch template
    process::Command::new("git")
        .arg("clone")
        .arg(URL)
        .arg(dir_path)
        .output()
        .expect(&format!(
            "Couldn't download template to directory \"{dir_path}\". Is git installed?"
        ));

    // rm -rf .git
    process::Command::new("rm")
        .arg("-rf")
        .arg(dir_path)
        .output()
        .expect("Couldn't delete the .git folder!");

    // move the stuff dir_path/* into .
    if mv {
        let mv_dir = dir_path.to_owned() + "/*";

        process::Command::new("mv")
            .arg(mv_dir)
            .arg(".")
            .output()
            .expect(&format!(
                "Couldn't download template to directory \"{dir_path}\". Is git installed?"
            ));
    }

    // replace text in build.rei & project.rei
    let build_rei = dir_path.to_owned() + "/build.rei";
    let project_rei = dir_path.to_owned() + "/project.rei";

    // NOTE: project name = dir_path pretty much
    // but also start a REPL to ask user to input the rest
    // or just leave for now

    // process::Command::new("sed")
    //     .arg("-i")
    //     .arg(format!("s/$require/{_str}/g"))
    //     .arg(build_rei)
    //     .output()
    //     .expect("Oops, couldnt replace the text in build.rei");

    // process::Command::new("sed")
    //     .arg("-i")
    //     .arg("s/$version/0.1/g")
    //     .arg(project_rei)
    //     .output()
    //     .unwrap();

    // bascially read_to_string, then use regex to find the start and end of the parts you want. And replace that part with your propaganda
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
        Commands::New { name } => generate_proj(&name.unwrap(), false),
        Commands::Init => generate_proj(".", true),
    }
}

#[test]
fn test_ui() {
    new_proj_ui();
}
