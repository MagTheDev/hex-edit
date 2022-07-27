
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read},
    path::PathBuf,
};

use anyhow::Result;
use anyhow::{bail, Context, Ok};
use console::{style};

const INNER_SEP: char = '┊';
const BORDER: char = '│';
const HORIZONTAL_LINE: char = '─';
const LEFT_BOTTOM: char = '└';
const BOTTOM_COLUMN_SEPARATOR: char = '┴';
const RIGHT_BOTTOM: char = '┘';
const LEFT_TOP: char = '┌';
const TOP_COLUMN_SEPARATOR: char = '┬';
const RIGHT_TOP: char = '┐';

/// Struct that represents the editor
/// This contains all the logic of the editor
/// ```rust
///     todo!("Add caching to Editor, so we dont have to rebuild the buffer every time we print")
/// ```
#[derive(Debug)]
pub struct Editor {
    pub data: Vec<u8>,
    _file: File,
    pub cursor: usize,
    update: bool,
    buffer: String,
}

impl Editor {
    pub fn new(file_location: PathBuf) -> Result<Self, anyhow::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_location)
            .context("Unable to open file")?;

        let mut buf_reader = BufReader::new(&file);
        let mut buffer = Vec::new();

        buf_reader
            .read_to_end(&mut buffer)
            .context("Unable to read file to buffer")?;

        Ok(Self {
            data: buffer,
            _file: file,
            update: true,
            cursor: 0,
            buffer: String::new(),
        })
    }

    pub fn move_cursor(&mut self, movement: Move) {
        match movement {
            Move::Up => {
                if self.cursor - 16 <= 0 {
                    return;
                }
                self.cursor -= 16;
            },
            Move::Down => {
                if self.cursor + 16 > self.data.len() {
                    return;
                }
                self.cursor += 16
            },
            Move::Left => {
                if self.cursor - 1 <= 0 {
                    return;
                }
                self.cursor -= 1;
            },
            Move::Right => {
                if self.cursor + 1 > self.data.len() {
                    return;
                }
                self.cursor += 1;
            },
        }
    }

    pub fn edit_at_cursor(&mut self, op: Operation) {
        
        match op {
            Operation::Insert(data) => {
                self.data.insert(self.cursor, data);
            },
            Operation::Delete => {
                self.data.remove(self.cursor);
            },
            Operation::Edit(data) => {
                self.data[self.cursor] = data;
            },
        }
        self.update = true;
    }

    pub fn edit(&mut self, index: usize, op: Operation) {

        match op {
            Operation::Insert(data) => {
                self.data.insert(index, data);
            },
            Operation::Delete => {
                self.data.remove(index);
            },
            Operation::Edit(data) => {
                self.data[index] = data;
            },
        }
        self.update = true;

    }

    pub fn update(&mut self) {
        // TODO: Add coloring/any indication to selected byte 
        // TODO: Optimize this maybe?

        // Wipe buffer
        self.buffer.clear();

        let mut output = String::new();

        for (mut index, item) in self.data.iter().enumerate() {

            index += 1;

            if index % 16 == 1 && index != 0 {
                output.push_str("\r\n");
            }
            
            if index % 8 == 1 && (index + 1) != 16 {
                output.push(' ');
            }

            let mut hex_repr = format!(" {:#02x}", item);            
            if hex_repr.len() < 5 {
                hex_repr = format!(" 0{:#02x}", item);
            }

            if self.cursor == (index - 1) {
                hex_repr = style(hex_repr).bright().cyan().to_string();
            } else {
                hex_repr = style(hex_repr).dim().color256(255).to_string()
            }

            output.push_str(&hex_repr);
            
        }
        self.update = false;
        self.buffer = output.replace("0x", "");

    }

    pub fn print(&mut self) -> String {
        if self.update {
            self.update();
        }
        self.buffer.clone()
    }
}

/// An enum that represents the operation user wants to make
pub enum Operation {
    Insert(u8),
    Delete,
    Edit(u8),
}

pub enum Move {
    Up,
    Down,
    Left,
    Right
}