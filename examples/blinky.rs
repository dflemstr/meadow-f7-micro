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

    let pins = meadow_f7_micro::pins::Pins::new(
        dp.GPIOA, dp.GPIOB, dp.GPIOC, dp.GPIOD, dp.GPIOE, dp.GPIOF, dp.GPIOG, dp.GPIOH, dp.GPIOI,
    );
    let clocks = meadow_f7_micro::clocks::configure(dp.RCC);

    let mut led_r = pins.onboard_led_red.into_push_pull_output();
    let mut led_g = pins.onboard_led_green.into_push_pull_output();
    let mut led_b = pins.onboard_led_blue.into_push_pull_output();
    let mut delay = meadow_f7_micro::hal::delay::Delay::new(cp.SYST, clocks);

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
