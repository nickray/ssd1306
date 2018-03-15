#![no_std]
// TODO: Docs
// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate embedded_graphics;
extern crate embedded_hal as hal;

use embedded_graphics::Drawing;
use embedded_graphics::drawable;
use hal::digital::OutputPin;
use hal::blocking::delay::DelayMs;
mod command;
use command::{AddrMode, Command, VcomhLevel};
mod interface;
use interface::DisplayInterface;

pub mod builder;
pub use builder::Builder;

pub struct SSD1306<DI> {
    iface: DI,
    buffer: [u8; 1024],
}

impl<DI> SSD1306<DI>
where
    DI: DisplayInterface,
{
    pub fn new(iface: DI) -> SSD1306<DI> {
        let mut disp = SSD1306 {
            iface,
            buffer: [0; 1024],
        };

        disp
    }

    /// Reset display
    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY)
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high();
        delay.delay_ms(1);
        rst.set_low();
        delay.delay_ms(10);
        rst.set_high();
    }

    pub fn flush(&mut self) {
        // TODO: Different width/height
        // let ecol = 128 - 1;
        // let epage = (64 / 8) - 1;

        // TODO: Fix these here
        // Command::ColumnAddress(0, ecol).send(&mut self.iface);
        // Command::PageAddress(0.into(), epage.into()).send(&mut self.iface);

        self.iface.send_data(&self.buffer);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let byte = &mut self.buffer[((y as usize) / 8 * 128) + (x as usize)];
        let bit = 1 << (y % 8);

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }

    // Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) {
        Command::DisplayOn(false).send(&mut self.iface);
        Command::DisplayClockDiv(0x8, 0x0).send(&mut self.iface);
        // TODO: What's this?
        // let mpx = 64 - 1;
        // Command::Multiplex(mpx).send(&mut self.iface);
        Command::DisplayOffset(0).send(&mut self.iface);
        Command::StartLine(0).send(&mut self.iface);
        // TODO: Ability to turn charge pump on/off
        Command::ChargePump(true).send(&mut self.iface);
        Command::AddressMode(AddrMode::Horizontal).send(&mut self.iface);
        Command::SegmentRemap(true).send(&mut self.iface);
        Command::ReverseComDir(true).send(&mut self.iface);

        // TODO: Display sizes
        // if self.width == 128 && self.height == 32 {
        //     Command::ComPinConfig(false, false).send(&mut self.iface);
        // } else if self.width == 128 && self.height == 64 {
        //     Command::ComPinConfig(true, false).send(&mut self.iface);
        // } else if self.width == 96 && self.height == 16 {
        //     Command::ComPinConfig(false, false).send(&mut self.iface);
        // }

        Command::Contrast(0x8F).send(&mut self.iface);
        Command::PreChargePeriod(0x1, 0xF).send(&mut self.iface);
        Command::VcomhDeselect(VcomhLevel::Auto).send(&mut self.iface);
        Command::AllOn(false).send(&mut self.iface);
        Command::Invert(false).send(&mut self.iface);
        Command::EnableScroll(false).send(&mut self.iface);
        Command::DisplayOn(true).send(&mut self.iface);
    }
}

impl<DI> Drawing for SSD1306<DI>
where
    DI: DisplayInterface,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel>,
    {
        for (pos, color) in item_pixels {
            self.set_pixel(pos.0, pos.1, color);
        }
    }
}
