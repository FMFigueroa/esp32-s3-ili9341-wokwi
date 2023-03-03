#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;

use hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay, Rtc, IO,
};

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    prelude::*,
    text::{Alignment, Text},
    Drawable,
};
use mipidsi::{Builder, ColorOrder, Orientation};

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

    //===Note: Backlight(LED) and RST pins are not available in the simulation with Wokwi. **===\\
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    backlight.set_high().unwrap();
    let rst = io.pins.gpio48.into_push_pull_output();
    let mut delay = Delay::new(&clocks);

    // Create Display Interface abstraction from SPI and DC pin
    let dc = io.pins.gpio4.into_push_pull_output();
    let di = SPIInterfaceNoCS::new(spi, dc);

    // Create the ILI9341 display driver in rgb565 color mode from the display interface
    // and use a HW reset pin during init, delay provider from your MCU
    let mut display = Builder::ili9341_rgb565(di)
        .with_orientation(Orientation::Portrait(true))
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(rst))
        .unwrap();
    //display.clear(Rgb565::BLACK).unwrap();


    Text::with_alignment("Hello World Rust!", Point::new(120, 180), MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(RgbColor::WHITE).build(),  Alignment::Center)
        .draw(&mut display)
        .unwrap();
    
    loop {}
}