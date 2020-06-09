#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

pub use hal::delay;
pub use stm32f7xx_hal as hal;

use hal::gpio;

pub struct MeadowF7Micro {
    pub pins: Pins,
    pub delay: hal::delay::Delay,
}

type DefaultPinState = gpio::Input<gpio::Floating>;

pub struct Pins {
    pub onboard_led_blue: gpio::gpioa::PA0<DefaultPinState>,
    pub onboard_led_green: gpio::gpioa::PA1<DefaultPinState>,
    pub onboard_led_red: gpio::gpioa::PA2<DefaultPinState>,

    pub d00: gpio::gpioi::PI9<DefaultPinState>,
    pub d01: gpio::gpioh::PH13<DefaultPinState>,
    pub d02: gpio::gpioc::PC6<DefaultPinState>,
    pub d03: gpio::gpiob::PB8<DefaultPinState>,
    pub d04: gpio::gpiob::PB9<DefaultPinState>,
    pub d05: gpio::gpioc::PC7<DefaultPinState>,
    pub d06: gpio::gpiob::PB0<DefaultPinState>,
    pub d07: gpio::gpiob::PB7<DefaultPinState>,
    // TODO: also SDA
    pub d08: gpio::gpiob::PB6<DefaultPinState>,
    // TODO: also SCL
    pub d09: gpio::gpiob::PB1<DefaultPinState>,
    pub d10: gpio::gpioh::PH10<DefaultPinState>,
    pub d11: gpio::gpioc::PC9<DefaultPinState>,
    pub d12: gpio::gpiob::PB14<DefaultPinState>,
    pub d13: gpio::gpiob::PB15<DefaultPinState>,
    pub d14: gpio::gpiog::PG3<DefaultPinState>,
    pub d15: gpio::gpioe::PE3<DefaultPinState>,

    pub a00: gpio::gpioa::PA4<DefaultPinState>,
    pub a01: gpio::gpioa::PA5<DefaultPinState>,
    pub a02: gpio::gpioa::PA3<DefaultPinState>,
    pub a03: gpio::gpioa::PA7<DefaultPinState>,
    pub a04: gpio::gpioc::PC0<DefaultPinState>,
    pub a05: gpio::gpioc::PC1<DefaultPinState>,

    pub sck: gpio::gpioc::PC10<DefaultPinState>,
    pub mosi: gpio::gpiob::PB5<DefaultPinState>,
    pub miso: gpio::gpioc::PC11<DefaultPinState>,

    pub esp_mosi: gpio::gpioi::PI3<DefaultPinState>,
    pub esp_miso: gpio::gpioi::PI2<DefaultPinState>,
    pub esp_clk: gpio::gpiod::PD3<DefaultPinState>,
    pub esp_cs: gpio::gpioi::PI0<DefaultPinState>,
    pub esp_boot: gpio::gpioi::PI10<DefaultPinState>,
    pub esp_rst: gpio::gpiof::PF7<DefaultPinState>,
    pub esp_uart_rx: gpio::gpiod::PD2<DefaultPinState>,
    pub esp_uart_tx: gpio::gpiob::PB13<DefaultPinState>,
}

impl MeadowF7Micro {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let dp = hal::device::Peripherals::take()?;
        Some(MeadowF7Micro::new(cp, dp))
    }

    pub fn new(core: cortex_m::Peripherals, device: hal::device::Peripherals) -> Self {
        use hal::gpio::GpioExt;
        use stm32f7xx_hal::rcc::RccExt;
        use stm32f7xx_hal::time::U32Ext;

        let rcc = device.RCC.constrain();
        // TODO: determine actual HSE value?
        let clocks = rcc
            .cfgr
            .sysclk(216.mhz())
            .freeze();

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();
        let gpiod = device.GPIOD.split();
        let gpioe = device.GPIOE.split();
        let gpiof = device.GPIOF.split();
        let gpiog = device.GPIOG.split();
        let gpioh = device.GPIOH.split();
        let gpioi = device.GPIOI.split();

        let pins = Pins {
            onboard_led_blue: gpioa.pa0,
            onboard_led_green: gpioa.pa1,
            onboard_led_red: gpioa.pa2,
            d00: gpioi.pi9,
            d01: gpioh.ph13,
            d02: gpioc.pc6,
            d03: gpiob.pb8,
            d04: gpiob.pb9,
            d05: gpioc.pc7,
            d06: gpiob.pb0,
            d07: gpiob.pb7,
            d08: gpiob.pb6,
            d09: gpiob.pb1,
            d10: gpioh.ph10,
            d11: gpioc.pc9,
            d12: gpiob.pb14,
            d13: gpiob.pb15,
            d14: gpiog.pg3,
            d15: gpioe.pe3,
            a00: gpioa.pa4,
            a01: gpioa.pa5,
            a02: gpioa.pa3,
            a03: gpioa.pa7,
            a04: gpioc.pc0,
            a05: gpioc.pc1,
            sck: gpioc.pc10,
            mosi: gpiob.pb5,
            miso: gpioc.pc11,
            esp_mosi: gpioi.pi3,
            esp_miso: gpioi.pi2,
            esp_clk: gpiod.pd3,
            esp_cs: gpioi.pi0,
            esp_boot: gpioi.pi10,
            esp_rst: gpiof.pf7,
            esp_uart_rx: gpiod.pd2,
            esp_uart_tx: gpiob.pb13,
        };

        let delay = hal::delay::Delay::new(core.SYST, clocks);

        Self { delay, pins }
    }
}
