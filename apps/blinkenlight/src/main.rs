#![no_std]
#![no_main]

use panic_halt as _;

//use hal::pac;
//use rp2040_hal as hal;
use rtt_target::{rprintln, rtt_init_print};

// The linker will place this boot block at the start of our program image. We
// need this to help the ROM bootloader get our code up and running.
// Note: This boot block is not necessary when using a rp-hal based BSP
// as the BSPs already perform this step.
#[link_section = ".boot_loader"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn main() -> ! {
    rtt_init_print!();

    loop {
        rprintln!("Hello World!");
    }
}
