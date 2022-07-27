use std::{
    path::PathBuf, thread::sleep, time::Duration, io::stdout,
};

use crossterm::event::KeyEvent;
use crossterm::terminal::disable_raw_mode;
use crossterm::{ExecutableCommand, terminal::{self, enable_raw_mode}, event::{read, Event, KeyCode, KeyModifiers}};
use editor::{Editor, Move};
use structopt::StructOpt;

pub mod editor;

#[allow(unused)]

// TODO: Fix buffer updates
// TODO: Add editing of bytes
// TODO: Add scrolling if the bytes cant fit on screen
// TODO: Add bottom status bar
// TODO: Add UTF-8 parsing of bytes
// TODO: Change coloring for better visibility

fn main() {
    let args = Args::from_args();

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
