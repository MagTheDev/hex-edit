use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};

use anyhow::Context;
use editor::Editor;
use structopt::StructOpt;

pub mod editor;

#[allow(unused)]

fn main() {
    let args = Args::from_args();

    // let file = OpenOptions::new()
    //     .read(true)
    //     .open(args.input)
    //     .context("Unable to read file").unwrap();

    // let mut reader = BufReader::new(file);
    // let mut buffer = Vec::new();

    // reader.read_to_end(&mut buffer).context("Unable to read file contents to buffer").unwrap();

    let mut editor = Editor::new(args.input).unwrap();
    println!("{}", editor.print())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "HeX Edit",
    about = "A simple program that lets you edit the binary of a file"
)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
