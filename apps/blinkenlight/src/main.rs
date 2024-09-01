#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::digital::OutputPin;
use hal::{pac, watchdog::Watchdog, Sio};
use rp2040_hal::{self as hal, Clock};

use rtt_target::{rprintln, rtt_init_print};

// The linker will place this boot block at the start of our program image. We
// need this to help the ROM bootloader get our code up and running.
// Note: This boot block is not necessary when using a rp-hal based BSP
// as the BSPs already perform this step.
#[link_section = ".boot_loader"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// External high-speed crystal on the pico board is 12Mhz
pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Initialize RTT printing
    rtt_init_print!();

    // Initialize system clocks
    rprintln!("INFO: Initializing system clocks & PLLs...");
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Initialize gpio bank0 pins
    rprintln!("INFO: Initializing GPIO bank0 pins...");
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // GPIO25 is connected to on-board LED
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);

        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
