

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

#[repr(transparent)] // ensure size of structure is the same size as its one field
struct Buffer {
  chars: [[ScreenChar; BUFFER_WIDTH], BUFFER_HEIGHT]
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




        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) {
    // Reset col position and go to next line in buffer
  }
}
