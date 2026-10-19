#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{self, pac, Clock};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led = pins.led.into_push_pull_output();
    rprintln!("DepthCast Pico: firmware started");

    loop {
        led.set_high().ok();
        rprintln!("DepthCast Pico: LED ON");
        delay.delay_ms(1100);
        led.set_low().ok();
        rprintln!("DepthCast Pico: LED OFF");
        delay.delay_ms(500);
    }
}
