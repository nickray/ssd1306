//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Run on a Blue Pill with `cargo run --example graphics_i2c_128x32`, currently only works on
//! nightly.

#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
// use cortex_m::asm::bkpt;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate panic_semihosting;
extern crate ssd1306;
//extern crate stm32f103xx_hal as blue_pill;
extern crate stm32l4xx_hal;

// use stm32l4xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32l4xx_hal::i2c::I2c;
use stm32l4xx_hal::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use rt::ExceptionFrame;
use ssd1306::prelude::*;
use ssd1306::Builder;

// use core::fmt::Write;
// extern crate cortex_m_semihosting as sh;
// use sh::hio;

#[entry]
fn main() -> ! {
    // let mut hstdout = hio::hstdout().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    // let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    // let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // let i2c = BlockingI2c::i2c1(
    //     dp.I2C1,
    //     (scl, sda),
    //     &mut afio.mapr,
    //     Mode::Fast {
    //         frequency: 400_000,
    //         duty_cycle: DutyCycle::Ratio2to1,
    //     },
    //     clocks,
    //     &mut rcc.apb1,
    //     1000,
    //     10,
    //     1000,
    //     1000,
    // );

    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        100.khz(),
        clocks,
        &mut rcc.apb1r1
    );

    // from original

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .with_i2c_addr(0x78)
        .connect_i2c(i2c)
        .into();

    disp.init().expect("disp.init failed");
    disp.flush().expect("disp.flush failed");

    let yoffset = 8;

    disp.draw(
        Line::new(
            Coord::new(8, 16 + yoffset),
            Coord::new(8 + 16, 16 + yoffset),
        ).with_stroke(Some(1u8.into()))
        .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8 + 16, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Rect::new(Coord::new(48, yoffset), Coord::new(48 + 16, 16 + yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Circle::new(Coord::new(96, yoffset + 8), 8)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
