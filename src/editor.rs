use std::{path::PathBuf, fs::{File, OpenOptions}, io::{BufReader, Read}, fmt::Display, };

use anyhow::{Context, bail, Ok};
use anyhow::Result;


const INNER_SEP: char = '┊';
const BORDER: char = '│';
const HORIZONTAL_LINE: char = '─';
const LEFT_BOTTOM: char = '└';
const BOTTOM_COLUMN_SEPARATOR: char = '┴';
const RIGHT_BOTTOM: char = '┘';
const LEFT_TOP: char = '┌';
const TOP_COLUMN_SEPARATOR: char = '┬';
const RIGHT_TOP:char = '┐';

/// Struct that represents the editor 
/// This contains all the logic of the editor
/// ```rust
///     todo!("Add caching to Editor, so we dont have to rebuild the buffer every time we print")
/// ```
#[derive(Debug)]
pub struct Editor {
    pub data: Vec<u8>,
    _file: File,
    cursor: usize,
    update: bool,
    buffer: String
}


impl Editor {

    pub fn new(file_location: PathBuf) -> Result<Self, anyhow::Error> {

        let file = OpenOptions::new()
            .read(true)
            .write(true).open(file_location).context("Unable to open file")?;

        let mut buf_reader = BufReader::new(&file);
        let mut buffer = Vec::new();

        buf_reader.read_to_end(&mut buffer).context("Unable to read file to buffer")?;

        Ok(Self {
            data: buffer,
            _file: file,
            update: true,
            cursor: 0,
            buffer: String::new(),
        })
    }

    pub fn move_cursor(&mut self, new_loc: usize) -> Result<()> {
        self.update = true;

        if new_loc > self.data.len() {
            bail!("Index out of range");
        }
        self.cursor = new_loc;
        Ok(())
    }

    pub fn edit_at_cursor(&mut self, op: Operation) {}

    pub fn edit(&mut self, index: usize, op: Operation) {}

    fn update(&mut self) {

        let mut output = String::new();
        for (index, item) in self.data.iter().enumerate() {

            if (index + 1) % 16 == 0 {
                output.push('\n');
            } else if (index + 1) % 8 == 0 {
                output.push(' ');
            } else {
                let to_push = format!("{:#02x} ", item);
                
                if to_push.len() < 5 {
                    let to_push = format!("0{}", to_push);
                    output.push_str(&to_push);
                } else {
                    output.push_str(&to_push);
                }
            }

        }
        self.update = false;
        self.buffer = output.replace("0x", "");

    }

    pub fn print(&self) -> String {
        self.buffer.clone()
    }   


}

impl Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print()) 
    }
}

/// An enum that represents the operation user wants to make
pub enum Operation {
    Insert(u8),
    Delete,
    Edit(u8)
}

pub enum Position<'a> {

    Header(usize),
    Footer(usize),
    Line(&'a [u8])
}