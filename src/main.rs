#![no_std]
#![no_main]

use esp_backtrace as _;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    Drawable,
    prelude::*
};
use tinytga::Tga;

use hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    spi::{Spi,SpiMode},
    timer::TimerGroup,
    Rtc,
    IO,
    Delay
};
use mipidsi::{ Orientation, ColorOrder};
use esp_println::println;

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    println!("Hello world rust!");

    //============= Display LCD TFT-ILI9341 240x320 with SPI Interface ============\\ 

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio36; // SPI Clock to LCD
    let mosi = io.pins.gpio35;  // SPI MOSI to LCD
 
    // configure SPI
    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        60u32.MHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    //===**⚠ the RST and backlight (LED) pins are not available in the simulation with Wokwi. **===\\
    let reset = io.pins.gpio48.into_push_pull_output();
    let mut backlight = io.pins.gpio45.into_push_pull_output(); 
    backlight.set_high().unwrap();

    // display interface abstraction from SPI and DC
    let dc = io.pins.gpio4.into_push_pull_output();
    let di = SPIInterfaceNoCS::new(spi, dc);

    let mut delay = Delay::new(&clocks);// delay
    // create driver
    let mut display = mipidsi::Builder::ili9341_rgb888(di)
        .with_orientation(Orientation::Portrait(true)) 
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(reset))
        .unwrap();
//======================================================================================================\\

// Load the TGA image.
    // The color type must be specified explicitly to match the color format used by the image,
    // otherwise the compiler may infer an incorrect type.
    let tga: Tga<Rgb888> = Tga::from_slice(include_bytes!("../assets/rust-pride.tga")).unwrap();

    // To draw the `tga` object to the display it needs to be wrapped in an `Image` object to set
    // the position at which it should drawn. Here, the top left corner of the image is set to
    // `(32, 32)`.
    let image = Image::new(&tga, Point::new(90, 140));

    // Display the image
    image.draw(&mut display).unwrap();

    
    loop {}
}
