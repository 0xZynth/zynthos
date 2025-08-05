#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zynthos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use x86_64::structures::paging::page_table::PageTableLevel;
use zynthos::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zynthos::hlt_loop();
}

#[cfg(test)]
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    zynthos::test_panic_handler(info);
}
/* OLD ENTRY POINT
use bootloader::BootInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    zynthos::init();
    
    use x86_64::registers::control::Cr3;
                        
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    zynthos::hlt_loop();
}*/

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

// new entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use zynthos::memory;
    use x86_64::{structures::paging::Translate, VirtAddr, structures::paging::Page};

    println!("Hello World{}", "!");
    zynthos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);}

    #[cfg(test)]
    test_main();

    println!("It did NOT crash");
    zynthos::hlt_loop(); 
}

