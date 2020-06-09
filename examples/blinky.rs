#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

use panic_semihosting as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = meadow_f7_micro::hal::device::Peripherals::take().unwrap();

    let device = meadow_f7_micro::MeadowF7Micro::new(cp, dp);

    let mut led_r = device.pins.onboard_led_red.into_push_pull_output();
    let mut led_g = device.pins.onboard_led_green.into_push_pull_output();
    let mut led_b = device.pins.onboard_led_blue.into_push_pull_output();
    let mut delay = device.delay;

    loop {
        led_r.set_high().unwrap();
        delay.delay_ms(1_000_u16);
        led_r.set_low().unwrap();
        led_g.set_high().unwrap();
        delay.delay_ms(1_000_u16);
        led_g.set_low().unwrap();
        led_b.set_high().unwrap();
        delay.delay_ms(1_000_u16);
        led_b.set_low().unwrap();
        delay.delay_ms(1_000_u16);
    }
}
