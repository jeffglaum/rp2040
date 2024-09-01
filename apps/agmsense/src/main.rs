#![no_std]
#![no_main]

use panic_halt as _;

use fugit::RateExtU32;

use embedded_hal::{digital::OutputPin, i2c::I2c};
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
    let mut led1_pin = pins.gpio24.into_push_pull_output();
    let mut led2_pin = pins.gpio25.into_push_pull_output();

    // Initialize i2c interface
    rprintln!("INFO: Initializing I2C interface...");
    let mut i2c = hal::I2C::i2c1(
        pac.I2C1,
        pins.gpio18.reconfigure(), // sda
        pins.gpio19.reconfigure(), // scl
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    // Invensense MPU9250A AGM is connected SDA=GPIO18, SCL=GPIO19 and I2C address is 0x68
    const MPU9250A_I2C_ADDRESS: u8 = 0x68;
    const MPU9250A_WHOAMI_REG: u8 = 0x75;
    let writebuf: [u8; 1] = [MPU9250A_WHOAMI_REG; 1];
    let mut readbuf: [u8; 1] = [0; 1];

    let result = i2c.write_read(MPU9250A_I2C_ADDRESS, &writebuf, &mut readbuf);
    if let Ok(_d) = result {
        // Expect WHO_AM_I to be 0x71
        rprintln!("INFO: MPU-9250A WHO_AM_I value={:#02x}", readbuf[0]);
    }

    loop {
        led1_pin.set_high().unwrap();
        led2_pin.set_low().unwrap();
        delay.delay_ms(500);

        led1_pin.set_low().unwrap();
        led2_pin.set_high().unwrap();
        delay.delay_ms(500);
    }
}
