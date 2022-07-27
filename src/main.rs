use std::{
    path::PathBuf, thread::sleep, time::Duration, io::stdout,
};

use crossterm::event::KeyEvent;
use crossterm::terminal::disable_raw_mode;
use crossterm::{ExecutableCommand, terminal::{self, enable_raw_mode}, execute, event::{read, Event, KeyCode, KeyModifiers}};
use editor::{Editor, Move};
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

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    enable_raw_mode();

    let mut editor = Editor::new(args.input).unwrap();
    println!("{}", editor.print());
    loop {

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE
            }) => {
                //println!("Key triggered");
                editor.move_cursor(Move::Up);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE
            }) => {
                //println!("Key triggered");
                editor.move_cursor(Move::Down);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE
            }) => {
                //println!("Key triggered");
                editor.move_cursor(Move::Right);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE
            }) => {
                //println!("Key triggered");
                editor.move_cursor(Move::Left);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE
            }) => {
                //println!("Key triggered");
                disable_raw_mode();
                break;
            }
            _ => {
                continue;
            }
        };
        // TODO: Fix updating the buffer
        editor.update();
        sleep(Duration::from_millis(50));
        println!("{}", editor.cursor);
        println!("{}", editor.print());
    }

}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Hex Edit",
    about = "A simple program that lets you edit the binary of a file"
)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
