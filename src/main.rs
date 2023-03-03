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
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::{Alignment, LineHeight, Text},
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
    let mosi = io.pins.gpio35; // SPI MOSI to LCD

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

    //Add Background White 
    Rectangle::new(Point::new(0, 0), Size::new(240, 320))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_color(Rgb565::WHITE)
                .fill_color(Rgb565::WHITE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();
    //Design Battery using Rectangle Primitives
    for position_x in (180..224).step_by(12) {
        Rectangle::new(Point::new(position_x, 24), Size::new(8, 16))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(2)
                    .stroke_color(Rgb565::CSS_DARK_SALMON)
                    .fill_color(Rgb565::CSS_AQUAMARINE)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();
    }
    Line::new(Point::new(226, 27), Point::new(226, 37))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::CSS_DARK_SALMON, 2))
        .draw(&mut display)
        .unwrap();

    // Add Button 
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::CSS_DARK_ORANGE)
        .stroke_width(3)
        .fill_color(Rgb565::CSS_GREEN_YELLOW)
        .build();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(75, 200), Size::new(100, 28)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    loop {}
}