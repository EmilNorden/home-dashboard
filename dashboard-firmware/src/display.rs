use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use epd_waveshare::epd7in5_v2::{Display7in5, Epd7in5};
use epd_waveshare::prelude::{Color, WaveshareDisplay};
use esp_hal::Blocking;
/*use epd_waveshare::color::Color;
use epd_waveshare::epd7in5_v2::{Display7in5, Epd7in5};
use epd_waveshare::prelude::WaveshareDisplay;*/
use esp_hal::gpio::{Input, Output};
use esp_hal::spi::master::Spi;
use esp_println::println;

#[embassy_executor::task]
pub async fn run(
    mut spi: SpiDevice<'static, NoopRawMutex, Spi<'static, Blocking>, Output<'static>>,
    dc: Output<'static>,
    rst: Output<'static>,
    busy: Input<'static>,
) {
    let mut delay = embassy_time::Delay {};
    let mut d = Epd7in5::new(&mut spi, busy, dc, rst, &mut delay, None).unwrap();
    let mut display = Display7in5::default();

    /*display.set_rotation(DisplayRotation::Rotate0);
    draw_text(&mut display, "Rotate 0!", 5, 50);

    display.set_rotation(DisplayRotation::Rotate90);
    draw_text(&mut display, "Rotate 90!", 5, 50);

    display.set_rotation(DisplayRotation::Rotate180);
    draw_text(&mut display, "Rotate 180!", 5, 50);

    display.set_rotation(DisplayRotation::Rotate270);
    draw_text(&mut display, "Rotate 270!", 5, 50);

    d.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
        .unwrap();
    Timer::after_millis(5000).await;*/

    // Draw some text
    println!("Print text in all sizes");
    // Color is inverted - black means white, white means black; the output will be black text on white background

    let font = &FONT_10X20;
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let mut i = 0;
    display.clear(Color::Black).ok();
    d.update_frame(&mut spi, display.buffer(), &mut delay)
        .unwrap();
    d.display_frame(&mut spi, &mut delay).unwrap();
    d.update_old_frame(&mut spi, display.buffer(), &mut delay)
        .unwrap();

    d.init_fast(&mut spi, &mut delay).unwrap();

    loop {
        if i < 8 {
            /*let _ = Text::with_text_style(
                &format!("Rust is awesome {}!", i),
                Point::new(20, 10),
                style,
                text_style,
            )
            .draw(&mut display);

            i += 1;

            d.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
                .unwrap();*/

            if i % 2 == 0 {
                let style = PrimitiveStyleBuilder::new()
                    .fill_color(Color::White)
                    .build();
                Rectangle::new(Point::new(0, 0), Size::new(100, 100))
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();
            } else {
                let style = PrimitiveStyleBuilder::new()
                    .fill_color(Color::Black)
                    .build();
                Rectangle::new(Point::new(0, 0), Size::new(100, 100))
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();
            }

            d.update_frame(&mut spi, display.buffer(), &mut delay)
                .unwrap();
            d.display_frame(&mut spi, &mut delay).unwrap();
            d.update_old_frame(&mut spi, display.buffer(), &mut delay)
                .unwrap();

            //d.display_frame(&mut spi, &mut delay).unwrap();
            println!("TICK {}", i);
            i += 1;

            Timer::after_millis(5000).await;
        } else {
            display.clear(Color::Black).ok();
            d.update_and_display_frame(&mut spi, display.buffer(), &mut delay)
                .unwrap();
            d.sleep(&mut spi, &mut delay).unwrap();
        }
    }
}

fn draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}
