mod display;
mod wifi_provisioning;

use crate::display::{display_wifi_provisioning2, render_text_view};
use crate::wifi_provisioning::{reset_provisioning, start_wifi_provisioning, wifi_is_provisioned};
use display::display_wifi_provisioning;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, TextStyleBuilder};
use epd_waveshare::epd7in5_v2::{Display7in5, Epd7in5};
use epd_waveshare::prelude::*;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::{AnyInputPin, InputOutput, Output, PinDriver, Pull};
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::reset::WakeupReason::Timer;
use esp_idf_svc::hal::spi::config::{BitOrder, Config, DriverConfig, Duplex, Mode};
use esp_idf_svc::hal::spi::Dma;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::sys::{
    esp_err_t, wifi_config_t, wifi_interface_t, wifi_prov_event_handler_t, wifi_prov_mgr_config_t,
    wifi_prov_mgr_init, wifi_prov_scheme_ble, wifi_prov_scheme_ble_event_cb_free_btdm,
};
use esp_idf_svc::sys::{wifi_prov_mgr_reset_provisioning, wifi_storage_t};
use esp_idf_svc::wifi::{BlockingWifi, Configuration, EspWifi};
use log::error;
use std::convert::Infallible;
use std::panic;

fn install_panic_hook() {
    panic::set_hook(Box::new(|info| {
        // `info` implements Display and includes payload + location
        // error!("PANIC: {info}");
        log::error!("PANIC: {info}");

        // If you want a best-effort backtrace on stable:
        // (Symbolization varies on embedded; still useful for PCs during dev.)
        let bt = std::backtrace::Backtrace::force_capture();
        //error!("Backtrace:\n{bt}");
        log::error!("Backtrace:\n{bt}");

        // Give UART a moment to flush (optional)
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Returning here continues the panic: with `panic=abort`, the runtime aborts.
    }));
}

extern "C" {
    pub fn esp_wifi_restore() -> esp_err_t;
    pub fn esp_wifi_set_storage(storage: wifi_storage_t) -> esp_err_t;
    pub fn esp_wifi_get_config(ifx: wifi_interface_t, conf: *mut wifi_config_t) -> esp_err_t;
}

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    install_panic_hook();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    //log::info!("Resetting...");
    //reset_provisioning()?;

    if !wifi_is_provisioned()? {
        log::info!("Wifi NOT provisioned!");
        start_wifi_provisioning("abc123", "Prov_HomeDashboard")?
    }
    log::info!("Wifi provisioned?");

    log::info!(
        "WIFI: {} {} {}",
        wifi.is_connected()?,
        wifi.is_up()?,
        wifi.is_connected()?
    );

    let mut spi = esp_idf_svc::hal::spi::SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio13,
        peripherals.pins.gpio14,
        None as Option<AnyInputPin>,
        &DriverConfig::default(),
    )?;
    let mut delay = Delay::default();

    let mut display_spi = esp_idf_svc::hal::spi::SpiDeviceDriver::new(
        &spi,
        Some(peripherals.pins.gpio15),
        &Config::default().write_only(true),
    )?;

    //display_busy.set_pull(Pull::Up)?;

    let mut device = Epd7in5::new(
        &mut display_spi,
        PinDriver::input(peripherals.pins.gpio25)?,
        PinDriver::output(peripherals.pins.gpio27)?,
        PinDriver::output(peripherals.pins.gpio26)?,
        &mut delay,
        None,
    )?;

    let mut display = Box::new(Display7in5::default());

    log::info!("Hello, world!");

    let font = &FONT_10X20;
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let mut i = 0;
    log::info!("About to clear!");
    display.clear(Color::Black).ok();
    log::info!("Cleared!!");

    device
        .update_frame(&mut display_spi, display.buffer(), &mut delay)
        .unwrap();
    device.display_frame(&mut display_spi, &mut delay).unwrap();
    device
        .update_old_frame(&mut display_spi, display.buffer(), &mut delay)
        .unwrap();

    device.init_fast(&mut display_spi, &mut delay).unwrap();

    //display_wifi_provisioning(display.as_mut(), "123", "123")?;

    /*let mut foo = FooTarget::new();
    display_wifi_provisioning2(&mut foo, "123", "123")?;
    foo.report();*/
    //display_wifi_provisioning2(display.as_mut(), "123", "123")?;
    //render_text_view(display.as_mut());
    // A. byte-aligned left edge and width
    /*Rectangle::new(Point::new(0, 0), Size::new(391, 391)) // 32 & 160 are multiples of 8
    .into_styled(PrimitiveStyle::with_fill(Color::White))
    .draw(display.as_mut())?;*/

    Rectangle::new(Point::new(0, 0), Size::new(420, 420))
        .into_styled(PrimitiveStyle::with_fill(Color::White))
        .draw(display.as_mut())?;

    // B. misaligned left edge but aligned width
    /*Rectangle::new(Point::new(30, 40), Size::new(160, 160)) // x=30 not multiple of 8
    .into_styled(PrimitiveStyle::with_fill(Color::White))
    .draw(display.as_mut())?;*/

    // C. aligned left edge but misaligned width
    /*Rectangle::new(Point::new(32, 40), Size::new(158, 160)) // width 158 not multiple of 8
    .into_styled(PrimitiveStyle::with_fill(Color::White))
    .draw(display.as_mut())?;*/

    device
        .update_frame(&mut display_spi, display.buffer(), &mut delay)
        .unwrap();
    device.display_frame(&mut display_spi, &mut delay).unwrap();
    device
        .update_old_frame(&mut display_spi, display.buffer(), &mut delay)
        .unwrap();

    loop {
        delay.delay_ms(1000);
    }

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
                Rectangle::new(Point::new(0, 0), Size::new(50, 150))
                    .into_styled(style)
                    .draw(display.as_mut())
                    .unwrap();
            } else {
                let style = PrimitiveStyleBuilder::new()
                    .fill_color(Color::Black)
                    .build();
                Rectangle::new(Point::new(0, 0), Size::new(50, 150))
                    .into_styled(style)
                    .draw(display.as_mut())
                    .unwrap();
            }

            device
                .update_frame(&mut display_spi, display.buffer(), &mut delay)
                .unwrap();
            device.display_frame(&mut display_spi, &mut delay).unwrap();
            device
                .update_old_frame(&mut display_spi, display.buffer(), &mut delay)
                .unwrap();

            //d.display_frame(&mut spi, &mut delay).unwrap();
            println!("TICK {}", i);
            i += 1;

            delay.delay_ms(5000);
        } else {
            display.clear(Color::Black).ok();
            device
                .update_and_display_frame(&mut display_spi, display.buffer(), &mut delay)
                .unwrap();
            device.sleep(&mut display_spi, &mut delay).unwrap();
            delay.delay_ms(5000);
        }
    }

    Ok(())
}

struct FooTarget {
    min: Point,
    max: Point,
}

impl FooTarget {
    pub fn new() -> Self {
        Self {
            min: Point::new(10000, 10000),
            max: Point::new(-10000, -10000),
        }
    }
    pub fn report(&self) {
        log::info!("FooTarget says: min {:?}, max {:?}", self.min, self.max);
    }
}

impl Dimensions for FooTarget {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(800, 480))
    }
}

impl DrawTarget for FooTarget {
    type Color = Color;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.min = p.0.min(self.min);
            self.max = p.0.max(self.max);
        }

        Ok(())
    }
}
