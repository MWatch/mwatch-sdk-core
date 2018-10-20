#![no_std]

extern crate stm32l432xx_hal as hal;

/// Type Alias to use in resource definitions
pub type Ssd1351 = ssd1351::mode::GraphicsMode<ssd1351::interface::SpiInterface<hal::spi::Spi<hal::stm32l4::stm32l4x2::SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5, hal::gpio::Input<hal::gpio::Floating>>>, hal::gpio::gpioa::PA6<hal::gpio::Alternate<hal::gpio::AF5, hal::gpio::Input<hal::gpio::Floating>>>, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5, hal::gpio::Input<hal::gpio::Floating>>>)>, hal::gpio::gpiob::PB1<hal::gpio::Output<hal::gpio::PushPull>>>>;

pub struct Context {
    pub display: Ssd1351
}

/// Pointer to the structure we're given by the host.
pub static mut TABLE_POINTER: Option<&'static Table> = None;

#[repr(C)]
/// The callbacks supplied by the OS.
pub struct Table {
    pub context: *mut Context,
    /// Draw a colour on the display - x, y, colour
    pub draw_pixel: extern "C" fn(*mut Context, u8, u8, u16) -> i32,
}

impl Table {
    pub fn get() -> &'static Table {
        unsafe {
            if let Some(tbl) = &TABLE_POINTER {
                tbl
            } else {
                panic!("Bad context");
            }
        }
    }
}
