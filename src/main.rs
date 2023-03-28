//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/pdm_table.rs"));
// include!("sine.rs");

use embedded_hal::digital::v2::OutputPin;
use rp_pico::entry;
// use seeeduino_xiao_rp2040::entry;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use seeeduino_xiao_rp2040 as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    // clocks::{init_clocks_and_plls, Clock},
    clocks::init_clocks_and_plls,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // let mut led_pin = pins.led_green.into_push_pull_output();
    // let mut led_pin = pins.led.into_push_pull_output();
    let mut led_pin = pins.gpio15.into_push_pull_output();

    loop {
        for val in PDM_TABLE {
            let mut v = val;
            for _ in 0..32 {
                if v & 1 == 1 {
                    led_pin.set_high().unwrap();
                } else {
                    led_pin.set_low().unwrap();
                }
                v >>= 1;
                // cortex_m::asm::delay(160);
            }
            cortex_m::asm::nop();
        }
        // info!("on!");
        // led_pin.set_high().unwrap();
        // delay.delay_ms(950);
        // info!("off!");
        // led_pin.set_low().unwrap();
        // delay.delay_ms(50);
    }
}

// End of file
