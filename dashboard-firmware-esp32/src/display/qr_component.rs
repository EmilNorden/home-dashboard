use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use qrcodegen::QrCode;
use std::borrow::Borrow;

pub struct QrComponent<C, Q: QrMatrix> {
    qr: Q,
    dark: C,
    light: C,
    module_size: u32,
    quiet_zone_size: u32,
    position: Point,
}

impl<C, Q: QrMatrix> QrComponent<C, Q>
where
    C: PixelColor,
{
    pub fn new(
        qr: Q,
        dark: C,
        light: C,
        module_size: u32,
        quiet_zone_size: Option<u32>,
        position: Point,
    ) -> Self {
        Self {
            qr,
            dark,
            light,
            module_size,
            quiet_zone_size: quiet_zone_size.unwrap_or(4),
            position,
        }
    }

    pub fn total_size(&self) -> u32 {
        let modules = self.qr.size() as u32 + (self.quiet_zone_size * 2);
        modules * self.module_size
    }
}

impl<C, Q: QrMatrix> Drawable for QrComponent<C, Q>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let total_size = self.total_size();
        Rectangle::new(self.position, Size::new_equal(total_size))
            .into_styled(PrimitiveStyle::with_fill(self.light))
            .draw(target)?;

        let modules = self.qr.size();

        let quiet_zone_offset = (self.quiet_zone_size * self.module_size) as i32;
        let module_size = self.module_size as i32;

        for y in 0..modules {
            for x in 0..modules {
                if self.qr.get_module(x, y) {
                    Rectangle::new(
                        Point::new(
                            quiet_zone_offset + (x * module_size),
                            quiet_zone_offset + (y * module_size),
                        ),
                        Size::new_equal(self.module_size),
                    )
                    .into_styled(PrimitiveStyle::with_fill(self.dark))
                    .draw(target)?;
                }
            }
        }

        Ok(())
    }
}

pub trait QrMatrix: Clone {
    fn size(&self) -> i32;
    fn get_module(&self, x: i32, y: i32) -> bool;
}

#[derive(Clone)]
pub struct QrCodeGenWrapper(pub QrCode);

impl QrMatrix for QrCodeGenWrapper {
    fn size(&self) -> i32 {
        self.0.size()
    }

    fn get_module(&self, x: i32, y: i32) -> bool {
        self.0.get_module(x, y)
    }
}
