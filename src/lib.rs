#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

pub mod clocks;
pub mod pins;

pub use stm32f7xx_hal as hal;

pub struct MeadowF7Micro {
    pub clocks: stm32f7xx_hal::rcc::Clocks,
    pub pins: pins::Pins,
}

impl MeadowF7Micro {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let dp = stm32f7xx_hal::device::Peripherals::take()?;
        Some(MeadowF7Micro::new(cp, dp))
    }

    pub fn new(_core: cortex_m::Peripherals, device: stm32f7xx_hal::device::Peripherals) -> Self {
        let clocks = clocks::configure(device.RCC);
        let pins = pins::Pins::new(
            device.GPIOA,
            device.GPIOB,
            device.GPIOC,
            device.GPIOD,
            device.GPIOE,
            device.GPIOF,
            device.GPIOG,
            device.GPIOH,
            device.GPIOI,
        );

        Self { clocks, pins }
    }
}
