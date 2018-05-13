use embedded_graphics::fonts::{Font, Font6x8};
use embedded_graphics::prelude::Transform;
use embedded_graphics::*;
use embedded_graphics::primitives::Rect;
use hal::gpio::gpioa::{PA5, PA6, PA7};
use hal::gpio::gpiob::PB1;
use hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use hal::spi::Spi;
use hal::stm32f103xx::SPI1;
use ssd1306::interface::SpiInterface;
use ssd1306::mode::GraphicsMode;

pub type OledDisplay = GraphicsMode<
    SpiInterface<
        Spi<
            SPI1,
            (
                PA5<Alternate<PushPull>>,
                PA6<Input<Floating>>,
                PA7<Alternate<PushPull>>,
            ),
        >,
        PB1<Output<PushPull>>,
    >,
>;

pub struct DisplayData {
    // Power
    pub local_bat: u8,  // %
    pub remote_bat: u8, // %
    pub current: f32,   // amperes
    pub mode: u8,       // beginner, eco etc

    // Distance
    pub speed: f32,              // m/s
    pub distance_travelled: u32, // metres
    pub distance_remaining: u32, // metres

    // Signal
    pub signal_strength: u8, // %
}

pub fn write_display(disp: &mut OledDisplay, data: DisplayData) {
    disp.draw(
        Font6x8::render_str(&format!("{}%", data.signal_strength))
            .translate((0, 40))
            .into_iter(),
    );

    disp.draw(Font6x8::render_str("TEST").translate((0, 50)).into_iter());


    // Remote battery
    disp.draw(Rect::new((111, 0), (126, 6), 1u8).into_iter());
    disp.set_pixel(127, 2, 1);
    disp.set_pixel(127, 3, 1);
    disp.set_pixel(127, 4, 1);
    let batt;
    if data.local_bat >= 100 {
        batt = 125;
    } else {
        batt = ((0.12f32 * data.local_bat as f32) as u32 + 113) as u8;
    }
    for x in 113..batt {
            disp.set_pixel(x.into(), 2, 1);
            disp.set_pixel(x.into(), 3, 1);
            disp.set_pixel(x.into(), 4, 1);
    }

    // Signal strength (TIDY UP AND MAKE IT LOOK BETTER)
    if data.signal_strength > 20 {
        for x in 0..2 {
            for y in 6..7 {
                disp.set_pixel(x, y, 1);
            }
        }
    }
    if data.signal_strength > 40 {
        for x in 3..5 {
            for y in 4..7 {
                disp.set_pixel(x, y, 1);
            }
        }
    }
    if data.signal_strength > 60 {
        for x in 6..8 {
            for y in 2..7 {
                disp.set_pixel(x, y, 1);
            }
        }
    }
    if data.signal_strength >= 80 {
        for x in 9..11 {
            for y in 0..7 {
                disp.set_pixel(x, y, 1);
            }
        }
    }
    if data.signal_strength <= 20 {
        // Make smaller font when available
        disp.draw(
            Font6x8::render_str("NO SIGNAL")
                .translate((0, 0))
                .into_iter(),
        );
    }
}
