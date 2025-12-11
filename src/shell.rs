use crate::{QemuExitCode, print, println, exit_qemu, vga_buffer};
use crate::interrupts;
use crate::allocator::ALLOCATOR;
use crate::fs::{FileSystem, ramfs::RamFS};
use crate::task::keyboard::SCANCODE_QUEUE;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Shell {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    buffer: String,
    fs: RamFS,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore),
            buffer: String::new(),
            fs: RamFS::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        println!("Welcome to ZynthOS");
        print!("> ");
        
        loop {
            let mut scancode = None;
            x86_64::instructions::interrupts::without_interrupts(|| {
                let mut queue = SCANCODE_QUEUE.lock();
                scancode = queue.pop();
            });

            if let Some(scancode) = scancode {
                if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                    if let Some(key) = self.keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::Unicode(character) => self.handle_char(character),
                            DecodedKey::RawKey(key) => {},
                        }
                    }
                }
            }
            
            x86_64::instructions::hlt();
        }
    }

    fn handle_char(&mut self, c: char) {
        match c {
            '\n' => {
                print!("\n");
                self.execute_command();
                self.buffer.clear();
                print!("> ");
            }
            '\x08' => {
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                    // Basic backspace handling for VGA buffer (move back, print space, move back)
                    // This is a bit hacky without a proper terminal driver, but works for simple cases
                    print!("{}", '\x08'); 
                }
            }
            _ => {
                self.buffer.push(c);
                print!("{}", c);
            }
        }
    }

    fn execute_command(&mut self) {
        let input = self.buffer.trim();
        if input.is_empty() {
            return;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "help" => {
                println!("Available commands:");
                println!("  help  - Show this help message");
                println!("  echo  - Echo the arguments");
                println!("  clear - Clear the screen (not implemented)");
                println!("  exit  - Exit the shell (& QEMU)");
            }
            "echo" => {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { print!(" "); }
                    print!("{}", arg);
                }
                println!("");
            }
            "exit" => {
                println!("Shutting Down");
                exit_qemu(QemuExitCode::Success);
            }
            "clear" => {
                vga_buffer::clear_screen();
            }
            "zfetch" => {
                let uptime = interrupts::uptime();
                let (used, total) = ALLOCATOR.lock().get_stats();
                
                println!("            .           User: root@zynthos");
                println!("           / \\          OS: ZynthOS v0.1.0");
                println!("          /   \\         Kernel: ZynthOS Kernel");
                println!("         /  |  \\        Uptime: {:.2}s", uptime);
                println!("        /   |   \\       Memory: {}B / {}B", used, total);
                println!("       /____|____\\      Shell: ZynthShell");
                println!("            |           ");
            }
            "ls" => {
                match self.fs.list_dir("/") {
                    Ok(entries) => {
                        for entry in entries {
                            println!("{}", entry);
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "touch" => {
                if let Some(filename) = args.get(0) {
                    match self.fs.create_file(filename) {
                        Ok(_) => println!("File created"),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    println!("Usage: touch <filename>");
                }
            }
            "mkdir" => {
                if let Some(dirname) = args.get(0) {
                    match self.fs.create_dir(dirname) {
                        Ok(_) => println!("Directory created"),
                        Err(e) => println!("Error: {:?}", e),
                    }
                } else {
                    println!("Usage: mkdir <dirname>");
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
