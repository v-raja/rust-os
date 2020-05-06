

#[allow(dead_code)] // switches off warnings for unused enums
#[derive(Debug, Clone, Copy, Eq, PartialEq)] // allows enum to be printed and compared
#[repr(u8)] // stores enum in 8 bits
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)] // allows enum to be printed and compared
#[repr(transparent)] // ensure size of structure is the same size as its one field
struct ColorCode(u8);

// First four bits in VGA describe foreground and last 4 describe background color
impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) <<  4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)] // allows enum to be printed and compared
#[repr(C)] // Enums with fields aren't a concept in C, so this bridges the enum to a valid C type
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)] // ensure size of structure is the same size as its one field
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


pub struct Writer {
  column_position: usize,     // current col position. Rows move when at end of row or on '\n' char
  color_code: ColorCode,      // color of the current character
  buffer: &'static mut Buffer // keep buffer alive for the entire duration of running the kernel
}

impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte  => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;
        let color_code = self.color_code;

        // Use Voltaile's write fn to write to buffer ao compiler doesn't optimize this write away
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code
        });

        self.column_position += 1;
      }
    }
  }

  // Reset col position and move everything one line up in buffer
  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
}


impl Writer {
  fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range, write a block
        _                   => self.write_byte(0xfe)
      }
    }
  }
}

// support Rust's built-in write!/writeln! formatting macros
use core::fmt;
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
  });
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) =>  {$crate::vga_buffer::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! println {
  () => {$crate::print!("\n")};
  ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)] // import these only for tests
use crate::{serial_print, serial_println};

#[test_case]
fn test_println_simple() {
    // Test if printing to VGA buffer works
    serial_print!("test_println... ");
    println!("test_println_simple output");
    serial_println!("[ok]");
}

#[test_case]
fn test_println_many() {
    serial_print!("test_println_many... ");
    for _ in 0..200 {
        println!("test_println_many output");
    }
    serial_println!("[ok]");
}

#[test_case]
fn test_println_output() {
  serial_print!("test_println_output...");

  let s = "This is a test string that fits on one line";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT-2][i].read();
    assert_eq!(char::from(screen_char.ascii_character), c);
  }

  serial_println!("[ok]");
}



