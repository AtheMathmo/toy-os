use core::ptr::Unique;
use core::fmt;
use core::fmt::Write;

use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::vga_buffer::WRITER.lock();
            writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

/// VGA Character Colours
#[allow(dead_code)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// A ColourCode
#[derive(Clone, Copy)]
struct ColourCode(u8);

impl ColourCode {
    /// Constructs a new ColourCode.
    const fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A Screen Character
#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode,
}

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A VGA Writer
///
/// A Writer that keeps track of it's position and allows
/// us to write characters on the screen.
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    /// Write a byte to screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    colour_code: self.colour_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Set the colour on the writer
    pub fn set_colour(&mut self, foreground: Colour, background: Colour) {
        let colour_code = ColourCode::new(foreground, background);
        self.colour_code = colour_code;
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            let buffer = self.buffer();
            buffer.chars[row] = buffer.chars[row + 1];
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            colour_code: self.colour_code,
        };

        self.buffer().chars[row] = [blank; BUFFER_WIDTH];
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    colour_code: ColourCode::new(Colour::LightCyan, Colour::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

/// Clears the screen to black.
pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

/// Prints an error to the screen in red.
///
/// Uses a new writer instance to prevent deadlocking.
/// 
/// # Safety
///
/// We create a new writer that requires a unique pointer
/// to the vga buffer. This introduces data races but shouldn't
/// do anything too dreadful...
pub unsafe fn print_error(fmt: fmt::Arguments) {
    use core::fmt::Write;

    let mut writer = Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Red, Colour::Black),
        buffer: Unique::new(0xb8000 as *mut _),
    };

    writer.new_line();
    writer.write_fmt(fmt).unwrap();
}
