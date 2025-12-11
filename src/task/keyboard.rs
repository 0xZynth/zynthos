use lazy_static::lazy_static;
use spin::Mutex;

pub struct ScancodeStream {
    buffer: [u8; 256],
    head: usize,
    tail: usize,
}

impl ScancodeStream {
    pub const fn new() -> Self {
        Self {
            buffer: [0; 256],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, scancode: u8) -> Result<(), ()> {
        if (self.tail + 1) % 256 == self.head {
            return Err(());
        }
        self.buffer[self.tail] = scancode;
        self.tail = (self.tail + 1) % 256;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            return None;
        }
        let scancode = self.buffer[self.head];
        self.head = (self.head + 1) % 256;
        Some(scancode)
    }
}

lazy_static! {
    pub static ref SCANCODE_QUEUE: Mutex<ScancodeStream> = Mutex::new(ScancodeStream::new());
}
