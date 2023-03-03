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
    prelude::*,
    mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    text::{Alignment, Text},
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    Drawable,
};
use embedded_plots::axis::{Axis, Scale, Placement};
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
        .with_orientation(Orientation::LandscapeInverted(false))
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(rst))
        .unwrap();
    //display.clear(Rgb565::BLACK).unwrap();


     Text::with_alignment("Temperature Logger", Point::new(160, 75), MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(RgbColor::CYAN).build(),  Alignment::Center)
        .draw(&mut display)
        .unwrap();

    // Design Batery
    for position_x in (280..304).step_by(6) {
        Rectangle::new(Point::new(position_x, 8), Size::new(6, 12))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb565::WHITE)
                    .fill_color(Rgb565::BLACK)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();
    }
    Line::new(Point::new(305, 12), Point::new(305, 16))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 3))
        .draw(&mut display)
        .unwrap();

    //========  Plot Axis ==========
        let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::WHITE)
        .build();

    Axis::new(0..50)
        .set_title("Temp/C")
        .set_scale(Scale::Fixed(10))
        .into_drawable_axis(Placement::Y { x: 40, y1: 90, y2: 220})
        .set_text_style(default_style)
        .set_color(RgbColor::RED)
        .draw(&mut display).unwrap();
    Axis::new(0..100)
        .set_title("Time/Seg")
        .set_scale(Scale::Fixed(10))
        .into_drawable_axis(Placement::X {x1: 40, x2: 280, y: 220})
        .set_text_style(default_style)
        .set_color(RgbColor::RED)
        .draw(&mut display).unwrap();

    for grid_y in (90..220).step_by(26){
        Line::new(Point::new(40, grid_y), Point::new(280, grid_y))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)
        .unwrap();
    } 
    for grid_x in (63..303).step_by(24) {
        Line::new(Point::new(grid_x, 92), Point::new(grid_x, 220))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display)
        .unwrap();
    }
        
    loop {}
}