#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;

use cortex_m::asm;
use panic_semihosting as _;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const MIN_BIT: u8 = 0;

struct Display {
    data: [[(u8, u8, u8, u8, u8, u8); WIDTH]; HEIGHT / 2],
    bit: u8,
}

pub const GAMMA: &[u8; 8] = &[0, 1, 5, 17, 37, 69, 115, 177];

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = meadow_f7_micro::hal::device::Peripherals::take().unwrap();

    let device = meadow_f7_micro::MeadowF7Micro::new(cp, dp);

    let mut led_clk = device.pins.d15.into_push_pull_output();
    let mut led_lat = device.pins.d00.into_push_pull_output();
    let mut led_oe = device.pins.d01.into_push_pull_output();
    let mut led_addr_a = device.pins.a05.into_push_pull_output();
    let mut led_addr_b = device.pins.a04.into_push_pull_output();
    let mut led_addr_c = device.pins.a03.into_push_pull_output();
    let mut led_addr_d = device.pins.a02.into_push_pull_output();
    let mut led_addr_e = device.pins.a01.into_push_pull_output();
    let mut led_r1 = device.pins.d10.into_push_pull_output();
    let mut led_g1 = device.pins.d09.into_push_pull_output();
    let mut led_b1 = device.pins.d11.into_push_pull_output();
    let mut led_r2 = device.pins.d13.into_push_pull_output();
    let mut led_g2 = device.pins.d12.into_push_pull_output();
    let mut led_b2 = device.pins.d14.into_push_pull_output();

    let mut display = Display {
        data: [[(0, 0, 0, 0, 0, 0); WIDTH]; HEIGHT / 2],
        bit: MIN_BIT,
    };

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            use micromath::F32Ext;

            let dx = 2.0 * x as f32 / WIDTH as f32 - 1.0;
            let dy = 2.0 * y as f32 / HEIGHT as f32 - 1.0;
            let hue = dy.atan2(dx) * 180.0 / core::f32::consts::PI;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance <= 1.0 {
                let color = palette::Srgb::from(palette::Hsv::new(hue, distance, 1.0 - distance));
                display.set_rgb(
                    x,
                    y,
                    (color.red * 255.0).round() as u8,
                    (color.green * 255.0).round() as u8,
                    (color.blue * 255.0).round() as u8,
                );
            }
        }
    }

    loop {
        let mut bit = display.bit;
        for (index, row) in display.data.iter().enumerate() {
            led_oe.set_high().unwrap();
            for &(r1, g1, b1, r2, g2, b2) in row.iter() {
                let mask = 1 << bit;
                set_pin(&mut led_r1, (r1 & mask) != 0);
                set_pin(&mut led_g1, (g1 & mask) != 0);
                set_pin(&mut led_b1, (b1 & mask) != 0);
                set_pin(&mut led_r2, (r2 & mask) != 0);
                set_pin(&mut led_g2, (g2 & mask) != 0);
                set_pin(&mut led_b2, (b2 & mask) != 0);
                led_clk.set_high().unwrap();
                asm::nop();
                led_clk.set_low().unwrap();
            }
            led_oe.set_high().unwrap();
            asm::nop();
            led_lat.set_low().unwrap();
            asm::nop();
            led_lat.set_high().unwrap();

            set_pin(&mut led_addr_a, index & 1 != 0);
            set_pin(&mut led_addr_b, index & 2 != 0);
            set_pin(&mut led_addr_c, index & 4 != 0);
            set_pin(&mut led_addr_d, index & 8 != 0);
            set_pin(&mut led_addr_e, index & 16 != 0);
            asm::nop();
            led_oe.set_low().unwrap();
            asm::delay(128 * GAMMA[display.bit as usize] as u32);
        }
        bit = bit + 1;
        if bit > 7 {
            bit = MIN_BIT;
        }
        display.bit = bit;
    }
}

fn set_pin(pin: &mut impl embedded_hal::digital::v2::OutputPin, value: bool) {
    let _ = if value { pin.set_high() } else { pin.set_low() };
}

impl Display {
    fn set_rgb(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let row = y % (HEIGHT / 2);
        let data = &mut self.data[row][x];
        if y < HEIGHT / 2 {
            data.0 = r as u8;
            data.1 = g as u8;
            data.2 = b as u8;
        } else {
            data.3 = r as u8;
            data.4 = g as u8;
            data.5 = b as u8;
        }
    }
}
