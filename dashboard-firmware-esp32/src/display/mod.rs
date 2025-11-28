use buoyant::layout::{Layout, VerticalAlignment};
use buoyant::render::Render;
use buoyant::render::Renderable;
mod layout;
mod qr_component;
mod render;

use crate::display::layout::qr::Qr;
use crate::display::layout::BigRect;
use crate::display::qr_component::{QrCodeGenWrapper, QrComponent};
use buoyant::environment::DefaultEnvironment;
use buoyant::render_target::EmbeddedGraphicsRenderTarget;
use buoyant::surface::{AsDrawTarget, Surface};
use buoyant::view::shape::Circle;
use buoyant::view::{HStack, Text, VStack, View, ViewExt};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, TextStyleBuilder};
use epd_waveshare::color::Color;
use qrcodegen::{QrCode, QrCodeEcc};
use std::error::Error;
use u8g2_fonts::{fonts, FontRenderer};

static LOGISO_32: FontRenderer = FontRenderer::new::<fonts::u8g2_font_logisoso32_tf>();

pub fn display_wifi_provisioning<D>(target: &mut D, pop: &str, ble_name: &str) -> anyhow::Result<()>
where
    D: DrawTarget<Color = Color>,
    <D as DrawTarget>::Error: Error + Send + Sync + 'static,
{
    //let qr = QrCode::encode_text(pop, QrCodeEcc::High)?;
    let qr = QrCode::encode_text("http://www.sweclockers.com", QrCodeEcc::High)?;

    QrComponent::new(
        QrCodeGenWrapper(qr),
        Color::White,
        Color::Black,
        10,
        None,
        Point::zero(),
    )
    .draw(target)?;

    /*let style = PrimitiveStyleBuilder::new()
            .fill_color(Color::White)
            .build();

        let block_size = 10;
        let screen_height = 480;
        let qr_screen_size = qr.size() * block_size;

        let start_x = 20;
        let start_y = (screen_height / 2) - (qr_screen_size / 2);

        for y in 0..qr.size() {
            for x in 0..qr.size() {
                let coordinate_x = start_x + x * block_size;
                let coordinate_y = start_y + y * block_size;
                if qr.get_module(x, y) {
                    Rectangle::new(
                        Point::new(coordinate_x, coordinate_y),
                        Size::new_equal(block_size as u32),
                    )
                    .into_styled(style)
                    .draw(target)?;
                }
            }
        }
    */
    let font = &profont::PROFONT_24_POINT;
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    /*Text::new(
        "Configure Wifi",
        Point::new( + qr_screen_size + 20, 30),
        style,
    )
    .draw(target)?;*/

    Ok(())
}
/*
fn wifi_provision_view() -> impl View<Color> {
    let qr = QrCode::encode_text("http://www.sweclockers.com", QrCodeEcc::High)?;

    HStack::new((
        Circle.stroked(15).foreground_color(Color::Black),
        QrComponent::new(
            &QrCodeGenWrapper(&qr),
            Color::White,
            Color::Black,
            10,
            None,
            Point::zero(),
        ),
    ))
}
*/
/*Qr {
qr: QrCodeGenWrapper(qr),
dark: BinaryColor::On,
light: BinaryColor::Off,
module_px: 10,
quiet_zone_modules: 4,
},*/

pub fn display_wifi_provisioning2<D>(target: &mut D, pop: &str, ble_name: &str)
where
    D: DrawTarget<Color = Color>,
    <D as DrawTarget>::Error: Error + Send + Sync + 'static,
{
    let view = view(ble_name);
    render_view(target, view);
    /*let resolved_layout = view.layout(
        &target.bounding_box().size.into(),
        &DefaultEnvironment::non_animated(),
    );
    let thing = view.render_tree(
        &resolved_layout,
        buoyant::primitives::Point::zero(),
        &DefaultEnvironment::non_animated(),
    );

    let bounds = target.bounding_box();
    let mut clipped = target.clipped(&bounds);

    let mut rt = EmbeddedGraphicsRenderTarget::new(&mut clipped);
    thing.render(&mut rt, &Color::White, buoyant::primitives::Point::zero());

    Ok(())*/
}

fn render_view<D>(target: &mut D, view: impl View<Color>)
where
    D: DrawTarget<Color = Color>,
    <D as DrawTarget>::Error: Error + Send + Sync + 'static,
{
    let resolved_layout = view.layout(
        &target.bounding_box().size.into(),
        &DefaultEnvironment::non_animated(),
    );
    let thing = view.render_tree(
        &resolved_layout,
        buoyant::primitives::Point::zero(),
        &DefaultEnvironment::non_animated(),
    );

    let bounds = target.bounding_box();
    let mut clipped = target.clipped(&bounds);

    let mut rt = EmbeddedGraphicsRenderTarget::new(&mut clipped);
    thing.render(&mut rt, &Color::White, buoyant::primitives::Point::zero());
}

pub fn render_text_view<D>(target: &mut D)
where
    D: DrawTarget<Color = Color>,
    <D as DrawTarget>::Error: Error + Send + Sync + 'static,
{
    render_view(target, foo_view());
}

fn text_view() -> impl View<Color> {
    Text::new(
        "Hello this is a long sentence. It could be longer though. A lot longer. This text wraps around it seems. Why does it not wrap around with the other view?",
        &FONT_10X20,
    )
}

fn vert_text_view() -> impl View<Color> {
    let qr = QrCode::encode_text("http://www.sweclockers.com", QrCodeEcc::High).unwrap();
    Qr {
        qr: QrCodeGenWrapper(qr),
        dark: Color::White,
        light: Color::Black,
        quiet_zone_modules: 4,
    }
}
//It could be longer though. A lot longer. This text wraps around it seems. Why does it not wrap around with the other view?
fn vert_text_view2() -> impl View<Color> {
    HStack::new((
        Circle.foreground_color(Color::White),
        VStack::new((Text::new("Hello", &FONT_10X20),
                     Text::new(
                         "Hello this is a long sentence. It could be longer though. A lot longer. This text wraps around it seems. Why does it not wrap around with the other view?",
                         &FONT_10X20,
                     )))
        ))
}

fn foo_view() -> impl View<Color> {
    BigRect {
        origin: buoyant::primitives::Point::new(0, 100),
        size: buoyant::primitives::Size::new(390, 391),
        color: Color::White,
    }
}

fn view(ble_name: &str) -> impl View<Color> {
    let qr = QrCode::encode_text("http://www.sweclockers.com", QrCodeEcc::High).unwrap();
    HStack::new((
        Qr {
            qr: QrCodeGenWrapper(qr),
            dark: Color::White,
            light: Color::Black,
            quiet_zone_modules: 4,
        },
        VStack::new((
            Text::new("Configure Wifi", &profont::PROFONT_24_POINT),
            Text::new("Scan the ", &FONT_10X20),
        ))
        .with_spacing(20),
    ))
    .with_alignment(VerticalAlignment::Center)
    .frame_sized(800, 480)
}

/*VStack::new((
    Text::new("Hello?", &profont::PROFONT_10_POINT),
    Text::new("World?", &profont::PROFONT_10_POINT),
)), */

struct EpdBinarySurface<'a, D>(&'a mut D);

// D is your epd_waveshare display/framebuffer that implements DrawTarget<Color = epd Color>.
/*impl<'a, D> Surface for EpdBinarySurface<'a, D>
where
    D: DrawTarget<Color = Color, Error = ()> + Dimensions,
{
    type Color = BinaryColor;

    fn size(&self) -> buoyant::primitives::Size {
        self.0.bounding_box().size.into()
    }

    fn draw_iter<I>(&mut self, pixels: I)
    where
        I: IntoIterator<Item = buoyant::primitives::Pixel<Self::Color>>,
    {
        self.0
            .draw_iter(pixels.into_iter().map(|p| {
                Pixel(
                    p.point.into(),
                    match p.color {
                        BinaryColor::Off => Color::Black,
                        BinaryColor::On => Color::White,
                    },
                )
            }))
            .unwrap();
    }
}
*/
