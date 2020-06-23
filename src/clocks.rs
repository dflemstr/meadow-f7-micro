pub fn configure(rcc_peripheral: stm32f7xx_hal::device::RCC) -> stm32f7xx_hal::rcc::Clocks {
    use stm32f7xx_hal::rcc;
    use stm32f7xx_hal::rcc::RccExt;
    use stm32f7xx_hal::time::U32Ext;

    // TODO: Is this truly the right HSE value?
    rcc_peripheral
        .constrain()
        .cfgr
        .sysclk(216.mhz())
        .hse(rcc::HSEClock::new(25.mhz(), rcc::HSEClockMode::Oscillator))
        .freeze()
}
