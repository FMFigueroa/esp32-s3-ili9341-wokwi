#![no_std]
#![no_main]

//This library provides smart pointers and collections to manage Values allocated to the heap.
extern crate alloc; 
use esp_backtrace as _;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    primitives::Rectangle,
    Drawable,
    prelude::*,
    text::Text,
};
use tinybmp::Bmp;

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



#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
fn init_heap() {
    const HEAP_SIZE: usize = 250 * 1024;

    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        assert!(
            heap_end - heap_start > HEAP_SIZE,
            "Not enough available heap memory."
        );
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    init_heap();
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
    let mut display = mipidsi::Builder::ili9341_rgb565(di)
        .with_orientation(Orientation::Portrait(true)) 
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(reset))
        .unwrap();

    // Load TGA file with the tiles.
    let image = Bmp::from_slice(include_bytes!("../assets/rust-pride.bmp")).unwrap();

    // Create sub images for the individual tiles.
    // Note that the tiles don't have to be the same size.
    let image_final = image.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(128, 128)));

    // Draw sub_image.
    Image::new(&image_final, Point::new(85, 80)).draw(&mut display).unwrap();
    
    // Draw labels.
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Rust Latam Group", Point::new(35, 70), text_style).draw(&mut display).unwrap();

    loop {}

}
