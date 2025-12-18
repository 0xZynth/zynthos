# ZynthOS

ZynthOS is a 64-bit operating system kernel written in Rust. This project explores low-level systems programming by building a functional "bare-metal" environment from scratch.

## 🚀 Key Features
### Bootloader integration
* configured to interface with UEFI via the ```bootloader``` crate

### Memory Management
* Implements paging to map virtual memory to physical addresses. 
* Heap allocation using a linked list allocator (and a bump allocator that is no longer in use)

### Hardware interfacing
* VGA Buffer: a simple driver for text output to screen with support for color.
* Serial communication: UART 16550 support

### Interrupt Handling
* Configured IDT (Interrupt Descriptor Table) to handle CPU exceptions (Breakpoints, Page Faults, Double Faults)
*  PIC (Programmable Interrupt Controller) management for hardware interrupts like the system timer and keyboard.

### ZynthShell
* A built-in command-line interface with basic commands like: ```help```, ```echo```, ```ls```, ```mkdir``` and a fetch utility ```zfetch```

### Filesystem
* A basic RamFS (in-memory filesystem) prototype that supports directory and file creation.

## 🛠️ Tech stack
* Language: Rust (Nightly used anywhere from 1.89.0 -> 1.94.0)
* Architecture: x86_64
* Tooling: ```cargo```, ```QEMU``` (for emulation)

## 📂 Project Structure
* ```src/main.rs```: Kernel entry point and initialization logic.
* ```src/interrupts.rs```: CPU Exceptiona nd hardware interrupt handlers.
* ```src/memory.rs```: Paging and physical memory management.
* ```src/allocator/```: 2 heap allocators (bump & linked list allcators)
* ```src/vga_buffer.rs```: 'screen' driver.
* ```src/shell.rs```: user interaction layer (with basic commands)

## 🏗️ Building and Running
To run ZynthOS in the QEMU emulator:
1. Ensure you have the Rust nightly toolchain installed.
2. clone the repository
3. Install all the tools & crates
4. run: ```cargo run``` 
* The kernel will boot, initialize the GDT & IDT, map the heap, and drop you into the shell.
![Screenshot of ZynthOS shell](https://raw.githubusercontent.com/0xZynth/zynthos/main/Assets/zos.png)
